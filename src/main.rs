use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    math::vec3,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices,
        render_resource::{AsBindGroup, PrimitiveTopology},
    },
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use fishy_assets::{
    CoralCollection, FishAnimationCollection, FishCollection, FishType, RockCollection,
    SeaweedCollection, ShellsCollection,
};
use input::{InputPlugin, Player, PlayerBundle, PlayerState, PlayerStateEvent};
use leafwing_input_manager::InputManagerBundle;
use noisy_bevy::{fbm_simplex_3d, NoisyShaderPlugin};
use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

use crate::fishy_assets::{CoralType, RockType, SeaweedType, ShellType};

mod compute_normals;
mod fishy_assets;
mod input;

fn main() {
    App::new()
        // Window resource
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fishy".to_string(), // ToDo
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                position: WindowPosition::At((0, 0).into()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(NoisyShaderPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(MaterialPlugin::<WaterMaterial>::default())
        // A deepwater blue
        .insert_resource(ClearColor(Color::rgb(0.6, 0.8, 9.0)))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, FishCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, FishAnimationCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, SeaweedCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, RockCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, CoralCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, ShellsCollection>(GameState::AssetLoading)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_system(setup_graphics.in_schedule(OnEnter(GameState::Playing)))
        .add_system(setup_level_gen.in_schedule(OnEnter(GameState::Playing)))
        .add_system(setup_player.in_schedule(OnEnter(GameState::Playing)))
        .add_system(setup_scene_once_loaded.in_set(OnUpdate(GameState::Playing)))
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Playing,
    GameOver,
}

#[derive(Component, Debug)]
struct Fish {
    pub fish_type: FishType,
}

#[derive(Bundle)]
struct FishBundle {
    fish: Fish,

    #[bundle]
    scene: SceneBundle,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for WaterMaterial {
    // fn fragment_shader() -> ShaderRef {
    //     "shaders/water_shader.wgsl".into()
    // }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct WaterMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    surface_y: f32,
    #[uniform(2)]
    wave_height: f32,
    #[uniform(3)]
    wave_length: f32,
    #[uniform(4)]
    wave_speed: f32,
}

fn setup_level_gen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    coral_collection: Res<CoralCollection>,
    rock_collection: Res<RockCollection>,
    seaweed_collection: Res<SeaweedCollection>,
    shells_collection: Res<ShellsCollection>,
) {
    const FREQUENCY_SCALE: f32 = 0.1;
    const AMPLITUDE_SCALE: f32 = 2.0;
    const RADIUS: f32 = 100.;
    const OCTAVES: usize = 3;
    const LACUNARITY: f32 = 1.5; // Increase this value to create more peaks
    const GAIN: f32 = 0.001; // Decrease this value to create more peaks

    let grid_half_size = RADIUS as i32 + 1;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for x in -grid_half_size..=grid_half_size {
        for z in -grid_half_size..=grid_half_size {
            let p = vec3(x as f32, 0.0, z as f32);
            let offset =
                fbm_simplex_3d(p * FREQUENCY_SCALE, OCTAVES, LACUNARITY, GAIN) * AMPLITUDE_SCALE;
            let height = offset + 0.5;

            // Add a new vertex at the calculated position.
            vertices.push([x as f32, height, z as f32]);
        }
    }

    // Generate indices
    for x in 0..(2 * grid_half_size) {
        for z in 0..(2 * grid_half_size) {
            let i = x * (2 * grid_half_size + 1) + z;
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 2 * grid_half_size + 2);

            indices.push(i);
            indices.push(i + 2 * grid_half_size + 2);
            indices.push(i + 2 * grid_half_size + 1);
        }
    }

    let mut terrain_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    terrain_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
    terrain_mesh.set_indices(Some(Indices::U32(
        indices.iter().map(|&i| i as u32).collect(),
    )));
    terrain_mesh.duplicate_vertices();
    compute_normals::compute_normals(&mut terrain_mesh);

    let y_offset = -16.0;

    const CORAL_TYPES: usize = 100;
    const ROCK_TYPES: usize = 80;
    const SEAWEED_TYPES: usize = 100;
    const SHELL_TYPES: usize = 60;

    let mut rng = thread_rng();
    let coral_types = CoralType::iter().collect::<Vec<_>>();
    let rock_types = RockType::iter().collect::<Vec<_>>();
    let seaweed_types = SeaweedType::iter().collect::<Vec<_>>();
    let shell_types = ShellType::iter().collect::<Vec<_>>();

    for _ in 0..CORAL_TYPES {
        let coral_type = coral_types.choose(&mut rng).unwrap();
        let coral_scene = coral_collection.model_for(coral_type);
        let scale = rng.gen_range(0.5..=4.0);
        let x = rng.gen_range(-RADIUS..=RADIUS);
        let z = rng.gen_range(-RADIUS..=RADIUS);
        // Just get N nearest vertices by x and z and take the average height
        let y = vertices
            .iter()
            .filter(|v| (v[0] - x as f32).abs() < 1.0 && (v[2] - z as f32).abs() < 1.0)
            .map(|v| v[1])
            .sum::<f32>()
            / 4.0;

        commands.spawn(SceneBundle {
            scene: coral_scene,
            transform: Transform::from_xyz(x as f32, y + y_offset - 2.0, z as f32)
                .with_scale(Vec3::splat(scale)),
            ..default()
        });
    }

    for _ in 0..ROCK_TYPES {
        let rock_type = rock_types.choose(&mut rng).unwrap();
        let rock_scene = rock_collection.model_for(rock_type);
        let scale = rng.gen_range(0.5..=4.0);
        let x = rng.gen_range(-RADIUS..=RADIUS);
        let z = rng.gen_range(-RADIUS..=RADIUS);
        // Just get N nearest vertices by x and z and take the average height
        let y = vertices
            .iter()
            .filter(|v| (v[0] - x as f32).abs() < 1.0 && (v[2] - z as f32).abs() < 1.0)
            .map(|v| v[1])
            .sum::<f32>()
            / 4.0;

        commands.spawn(SceneBundle {
            scene: rock_scene,
            transform: Transform::from_xyz(x as f32, y + y_offset - 2.0, z as f32)
                .with_scale(Vec3::splat(scale)),
            ..default()
        });
    }

    for _ in 0..SEAWEED_TYPES {
        let seaweed_type = seaweed_types.choose(&mut rng).unwrap();
        let seaweed_scene = seaweed_collection.model_for(seaweed_type);
        let scale = rng.gen_range(0.5..=6.0);
        let x = rng.gen_range(-RADIUS..=RADIUS);
        let z = rng.gen_range(-RADIUS..=RADIUS);
        // Just get N nearest vertices by x and z and take the average height
        let y = vertices
            .iter()
            .filter(|v| (v[0] - x as f32).abs() < 1.0 && (v[2] - z as f32).abs() < 1.0)
            .map(|v| v[1])
            .sum::<f32>()
            / 4.0;

        commands.spawn(SceneBundle {
            scene: seaweed_scene,
            transform: Transform::from_xyz(x as f32, y + y_offset - 2.0, z as f32)
                .with_scale(Vec3::splat(scale)),
            ..default()
        });
    }

    for _ in 0..SHELL_TYPES {
        let shell_type = shell_types.choose(&mut rng).unwrap();
        let shell_scene = shells_collection.model_for(shell_type);
        let scale = rng.gen_range(0.5..=2.0);
        let x = rng.gen_range(-RADIUS..=RADIUS);
        let z = rng.gen_range(-RADIUS..=RADIUS);
        // Just get N nearest vertices by x and z and take the average height
        let y = vertices
            .iter()
            .filter(|v| (v[0] - x as f32).abs() < 1.0 && (v[2] - z as f32).abs() < 1.0)
            .map(|v| v[1])
            .sum::<f32>()
            / 4.0;

        commands.spawn(SceneBundle {
            scene: shell_scene,
            transform: Transform::from_xyz(x as f32, y + y_offset, z as f32)
                .with_scale(Vec3::splat(scale)),
            ..default()
        });
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(terrain_mesh.clone()),
        transform: Transform::from_xyz(0.0, y_offset, 0.0 as f32),
        material: materials.add(StandardMaterial {
            // solid White
            // base_color: Color::hex("ffffff").unwrap(),
            // dark blue
            base_color: Color::hex("0a0a2c").unwrap(),
            perceptual_roughness: 0.8,
            ..default()
        }),
        ..default()
    });
}

fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 0.01).looking_at(Vec3::ZERO, Vec3::Y),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 30.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            // Deep water blue
            color: Color::hex("0a0a2c").unwrap(),
            intensity: 100000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        ..default()
    });

    // commands.insert_resource(AmbientLight {
    //     color: Color::ORANGE_RED,
    //     brightness: 1.0,
    // });

    // commands
    //     .spawn(SpotLightBundle {
    //         transform: Transform::from_xyz(0.0, 10.0, 0.0)
    //             .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
    //         spot_light: SpotLight {
    //             intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
    //             color: Color::RED,
    //             shadows_enabled: true,
    //             inner_angle: 0.6,
    //             outer_angle: 0.8,
    //             ..default()
    //         },
    //         ..default()
    //     });

    // Bevy is a right handed, Y-up system.
    commands.spawn((
        Camera3dBundle {
            tonemapping: Tonemapping::TonyMcMapface,
            // projection: Projection::Orthographic(OrthographicProjection {
            //     scale: 12.0,
            //     scaling_mode: ScalingMode::FixedVertical(4.0),
            //     ..default()
            // }),
            transform: Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // FogSettings {
        //     // A greenish blue fog
        //     color: Color::rgba(0.0, 0.5, 0.8, 1.0),
        //     falloff: FogFalloff::Atmospheric {
        //         extinction: Vec3::splat(0.015),
        //         inscattering: Vec3::splat(0.007),
        //     },
        //     ..default()
        // },
        BloomSettings::default(),
    ));

    // Spawn a simple plane for a water shader
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 30.0,
            ..Default::default()
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: water_materials.add(WaterMaterial {
            time: 0.0,
            surface_y: 0.5,
            wave_height: 0.1,
            wave_length: 0.5,
            wave_speed: 0.2,
        }),
        ..default()
    });
}

fn setup_player(mut commands: Commands, fish_collection: Res<FishCollection>) {
    let fish_type = FishType::Turtle;
    // Scale up by two cause he's smol
    let transform = Transform::from_xyz(0.0, 0.0, 0.01).with_scale(Vec3::splat(2.0));

    commands.spawn((
        FishBundle {
            fish: Fish { fish_type },
            scene: SceneBundle {
                scene: fish_collection.model_for(&fish_type),
                transform,
                ..default()
            },
        },
        PlayerBundle {
            player: Player::default(),
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                ..default()
            },
        },
    ));
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animation_collection: Res<FishAnimationCollection>,
    mut player_query: Query<&mut AnimationPlayer>,
    fish_query: Query<&Fish>,
    mut player_state_events: EventReader<PlayerStateEvent>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    for PlayerStateEvent { state, entity } in player_state_events.iter() {
        let fish = fish_query.get(*entity).unwrap();
        let fish_animations = animation_collection.animations_for(&fish.fish_type);

        match state {
            PlayerState::Idle => {
                let idle_animation = fish_animations.idle;

                player.play(idle_animation).repeat();
            }
            PlayerState::Moving { direction: _ } => {
                let Some(moving_animation) = fish_animations.moving else {
                    continue;
                };

                player.play(moving_animation).repeat();
            }
        }
    }
}
