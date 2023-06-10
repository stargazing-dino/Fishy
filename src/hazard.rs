use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    fishy_assets::{FishAnimationCollection, FishAnimations, FishCollection, FishType},
    Bounds, Fish, FishBundle, GameState, InitialAnimation, SimulationSet,
};

pub struct HazardPlugin;

impl Plugin for HazardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HazardSpawnTimer>().add_systems(
            (
                tick_hazard_spawn_timer,
                spawn_hazard,
                despawn_hazard,
                move_hazard,
            )
                .distributive_run_if(in_state(GameState::Playing))
                .in_set(SimulationSet::Logic),
        );
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Hazard {
    speed: f32,
}

impl Default for Hazard {
    fn default() -> Hazard {
        Hazard { speed: 0.0 }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
enum HazardType {
    Crab,
    Squid,
    Octopus,
    Hammerhead,
    Eel,
}

impl HazardType {
    pub fn into_fish_type(self) -> Option<FishType> {
        match self {
            HazardType::Crab => Some(FishType::Crab),
            HazardType::Eel => Some(FishType::Eel),
            HazardType::Hammerhead => Some(FishType::Hammerhead),
            HazardType::Octopus => Some(FishType::Octopus),
            HazardType::Squid => Some(FishType::Squid),
            // In the case where a hazard is not a fish, we'll have to deal with that in the
            // future by expanding this and the function defs below.
            _ => None,
        }
    }

    pub fn model_from(&self, collection: &FishCollection) -> Handle<Scene> {
        self.into_fish_type()
            .and_then(|fish_type| Some(fish_type.model_from(collection)))
            .unwrap()
    }

    pub fn animations_from(&self, collection: &FishAnimationCollection) -> FishAnimations {
        self.into_fish_type()
            .and_then(|fish_type| Some(fish_type.animations_from(collection)))
            .unwrap()
    }
}

#[derive(Resource)]
pub struct HazardSpawnTimer {
    pub timer: Timer,
}

impl Default for HazardSpawnTimer {
    fn default() -> HazardSpawnTimer {
        HazardSpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

pub fn tick_hazard_spawn_timer(mut hazard_spawn_timer: ResMut<HazardSpawnTimer>, time: Res<Time>) {
    hazard_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_hazard(
    mut commands: Commands,
    bounds: Res<Bounds>,
    fish_collection: Res<FishCollection>,
    animation_collection: Res<FishAnimationCollection>,
    hazard_spawn_timer: Res<HazardSpawnTimer>,
) {
    if !hazard_spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let hazard_types = HazardType::iter().collect::<Vec<_>>();
    let hazard_type = hazard_types.choose(&mut rng).unwrap();
    // let hazard_type = HazardType::Eel;
    let spawn_left = rng.gen_bool(0.5);
    let mut speed = rng.gen_range(1.0..3.0);
    let x = if spawn_left {
        speed *= 1.0;
        bounds.min.x
    } else {
        speed *= -1.0;
        bounds.max.x
    };
    let animations = hazard_type.animations_from(&animation_collection);
    let animation = animations.moving.unwrap_or(animations.idle);

    // TODO: This will not always be true once you add different hazards!
    let fish_type = hazard_type.into_fish_type().unwrap();

    // We need to spawn things off camera
    // Crabs on the ground
    // Eels spawn near the bottom
    // Everything else is free game
    let (transform, speed_multiplier) = match hazard_type {
        HazardType::Crab => {
            let bottom = (bounds.max.y - bounds.min.y) / 5.0;
            let y = rng.gen_range(bounds.min.y..bounds.min.y + bottom);
            let transform = Transform::from_xyz(x, y, 0.0);

            (transform, 1.0)
        }
        HazardType::Eel => {
            let bottom = (bounds.max.y - bounds.min.y) / 3.0;
            let y = rng.gen_range(bounds.min.y..bounds.min.y + bottom);
            let mut transform = Transform::from_xyz(x, y, 0.0);

            if spawn_left {
                transform.rotate_y(PI / 2.0);
            } else {
                transform.rotate_y(-PI / 2.0);
            }

            (transform, 1.25)
        }
        HazardType::Hammerhead => {
            let top = (bounds.max.y - bounds.min.y) / 2.0;
            let y = rng.gen_range(bounds.min.y + top..bounds.max.y);
            let mut transform = Transform::from_xyz(x, y, 0.0);

            if spawn_left {
                transform.rotate_y(-PI / 2.0);
            } else {
                transform.rotate_y(PI / 2.0);
            }

            (transform, 2.0)
        }
        _ => {
            let top = (bounds.max.y - bounds.min.y) / 2.0;
            let y = rng.gen_range(bounds.min.y + top..bounds.max.y);
            let mut transform = Transform::from_xyz(x, y, 0.0);
            if spawn_left {
                transform.rotate_y(PI / 2.0);
            } else {
                transform.rotate_y(-PI / 2.0);
            }

            (transform, 1.5)
        }
    };

    commands.spawn((
        InitialAnimation {
            animation,
            repeat: true,
        },
        FishBundle {
            fish: Fish { fish_type },
            scene: SceneBundle {
                scene: fish_type.model_from(&fish_collection),
                transform,
                ..default()
            },
        },
        Hazard {
            speed: speed * speed_multiplier,
        },
    ));
}

// Just moves the fish horizontally across the screen
pub fn move_hazard(mut query: Query<(&mut Transform, &Hazard), With<Fish>>, time: Res<Time>) {
    for (mut transform, hazard) in query.iter_mut() {
        let speed = hazard.speed;

        transform.translation.x += speed * time.delta_seconds();
    }
}

pub fn despawn_hazard(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Hazard>>,
    bounds: Res<Bounds>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < bounds.min.x || transform.translation.x > bounds.max.x {
            commands.entity(entity).despawn_recursive();
        } else if transform.translation.y < bounds.min.y || transform.translation.y > bounds.max.y {
            commands.entity(entity).despawn_recursive();
        }
    }
}
