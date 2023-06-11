#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::{AnimationClip, Image};
use bevy::{
    prelude::{Handle, Resource},
    scene::Scene,
};
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum CoralType {
    Coral,
    Coral1,
    Coral2,
    Coral3,
    Coral4,
    Coral5,
    Coral6,
}

#[derive(AssetCollection, Resource)]
pub struct CoralCollection {
    #[asset(path = "models/Coral.glb#Scene0")]
    pub coral: Handle<Scene>,

    #[asset(path = "models/Coral1.glb#Scene0")]
    pub coral_1: Handle<Scene>,

    #[asset(path = "models/Coral2.glb#Scene0")]
    pub coral_2: Handle<Scene>,

    #[asset(path = "models/Coral3.glb#Scene0")]
    pub coral_3: Handle<Scene>,

    #[asset(path = "models/Coral4.glb#Scene0")]
    pub coral_4: Handle<Scene>,

    #[asset(path = "models/Coral5.glb#Scene0")]
    pub coral_5: Handle<Scene>,

    #[asset(path = "models/Coral6.glb#Scene0")]
    pub coral_6: Handle<Scene>,
}

impl CoralType {
    /// Gets the corresponding coral model for the given coral type
    pub fn model_from(&self, collection: &CoralCollection) -> Handle<Scene> {
        match self {
            CoralType::Coral => collection.coral.clone(),
            CoralType::Coral1 => collection.coral_1.clone(),
            CoralType::Coral2 => collection.coral_2.clone(),
            CoralType::Coral3 => collection.coral_3.clone(),
            CoralType::Coral4 => collection.coral_4.clone(),
            CoralType::Coral5 => collection.coral_5.clone(),
            CoralType::Coral6 => collection.coral_6.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum RockType {
    Rock,
    Rock1,
    Rock2,
    Rock3,
    Rock4,
    Rock5,
    Rock6,
    Rock7,
    Rock8,
    Rock9,
    Rock10,
}

#[derive(AssetCollection, Resource)]
pub struct RockCollection {
    #[asset(path = "models/ROck.glb#Scene0")]
    rock: Handle<Scene>,

    #[asset(path = "models/Rock1.glb#Scene0")]
    rock_1: Handle<Scene>,

    #[asset(path = "models/Rock2.glb#Scene0")]
    rock_2: Handle<Scene>,

    #[asset(path = "models/Rock3.glb#Scene0")]
    rock_3: Handle<Scene>,

    #[asset(path = "models/Rock4.glb#Scene0")]
    rock_4: Handle<Scene>,

    #[asset(path = "models/Rock5.glb#Scene0")]
    rock_5: Handle<Scene>,

    #[asset(path = "models/Rock6.glb#Scene0")]
    rock_6: Handle<Scene>,

    #[asset(path = "models/Rock7.glb#Scene0")]
    rock_7: Handle<Scene>,

    #[asset(path = "models/Rock8.glb#Scene0")]
    rock_8: Handle<Scene>,

    #[asset(path = "models/Rock9.glb#Scene0")]
    rock_9: Handle<Scene>,

    #[asset(path = "models/Rock10.glb#Scene0")]
    rock_10: Handle<Scene>,
}

impl RockType {
    /// Gets the corresponding rock model for the given rock type
    pub fn model_from(&self, collection: &RockCollection) -> Handle<Scene> {
        match self {
            RockType::Rock => collection.rock.clone(),
            RockType::Rock1 => collection.rock_1.clone(),
            RockType::Rock2 => collection.rock_2.clone(),
            RockType::Rock3 => collection.rock_3.clone(),
            RockType::Rock4 => collection.rock_4.clone(),
            RockType::Rock5 => collection.rock_5.clone(),
            RockType::Rock6 => collection.rock_6.clone(),
            RockType::Rock7 => collection.rock_7.clone(),
            RockType::Rock8 => collection.rock_8.clone(),
            RockType::Rock9 => collection.rock_9.clone(),
            RockType::Rock10 => collection.rock_10.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum SeaweedType {
    // This seaweed is super janky
    // Seaweed,
    Seaweed1,
    Seaweed2,
}

#[derive(AssetCollection, Resource)]
pub struct SeaweedCollection {
    // #[asset(path = "models/Seaweed.glb#Scene0")]
    // seaweed: Handle<Scene>,
    #[asset(path = "models/Seaweed1.glb#Scene0")]
    seaweed_1: Handle<Scene>,

    #[asset(path = "models/Seaweed2.glb#Scene0")]
    seaweed_2: Handle<Scene>,
}

impl SeaweedType {
    /// Gets the corresponding seaweed model for the given seaweed type
    pub fn model_from(&self, collection: &SeaweedCollection) -> Handle<Scene> {
        match self {
            // SeaweedType::Seaweed => collection.seaweed.clone(),
            SeaweedType::Seaweed1 => collection.seaweed_1.clone(),
            SeaweedType::Seaweed2 => collection.seaweed_2.clone(),
        }
    }

    pub fn animation_from(&self, collection: &SeaweedAnimationCollection) -> Handle<AnimationClip> {
        match self {
            // SeaweedType::Seaweed => collection.seaweed.clone(),
            SeaweedType::Seaweed1 => collection.seaweed_1.clone(),
            SeaweedType::Seaweed2 => collection.seaweed_2.clone(),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct SeaweedAnimationCollection {
    // #[asset(path = "models/Seaweed.glb#Animation0")]
    // pub seaweed: Handle<AnimationClip>,
    #[asset(path = "models/Seaweed1.glb#Animation0")]
    pub seaweed_1: Handle<AnimationClip>,

    #[asset(path = "models/Seaweed2.glb#Animation0")]
    pub seaweed_2: Handle<AnimationClip>,
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ShellType {
    Shell,
    Shell1,
    Shell2,
    Shell3,
}

#[derive(AssetCollection, Resource)]
pub struct ShellsCollection {
    #[asset(path = "models/Shells.glb#Scene0")]
    shells: Handle<Scene>,

    #[asset(path = "models/Shells1.glb#Scene0")]
    shells_1: Handle<Scene>,

    #[asset(path = "models/Shells2.glb#Scene0")]
    shells_2: Handle<Scene>,

    #[asset(path = "models/Shells3.glb#Scene0")]
    shells_3: Handle<Scene>,
}

impl ShellType {
    /// Gets the corresponding shell model for the given shell type
    pub fn model_from(&self, collection: &ShellsCollection) -> Handle<Scene> {
        match self {
            ShellType::Shell => collection.shells.clone(),
            ShellType::Shell1 => collection.shells_1.clone(),
            ShellType::Shell2 => collection.shells_2.clone(),
            ShellType::Shell3 => collection.shells_3.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum FishType {
    BrownFish,
    ClownFish,
    Crab,
    DoryFish,
    Eel,
    Hammerhead,
    Lobster,
    Octopus,
    Penguin,
    Seal,
    Squid,
    StarFish,
    StingRay,
    TunaFish,
    Turtle,
    Whale,
}

#[derive(AssetCollection, Resource)]
pub struct FishCollection {
    #[asset(path = "models/BrownFish.glb#Scene0")]
    pub brown_fish: Handle<Scene>,

    #[asset(path = "models/ClownFish.glb#Scene0")]
    pub clown_fish: Handle<Scene>,

    #[asset(path = "models/Crab.glb#Scene0")]
    pub crab: Handle<Scene>,

    #[asset(path = "models/DoryFish.glb#Scene0")]
    pub dory_fish: Handle<Scene>,

    #[asset(path = "models/Eel.glb#Scene0")]
    pub eel: Handle<Scene>,

    #[asset(path = "models/Hammerhead.glb#Scene0")]
    pub hammerhead: Handle<Scene>,

    #[asset(path = "models/Lobster.glb#Scene0")]
    pub lobster: Handle<Scene>,

    #[asset(path = "models/Octopus.glb#Scene0")]
    pub octopus: Handle<Scene>,

    #[asset(path = "models/Penguin.glb#Scene0")]
    pub penguin: Handle<Scene>,

    #[asset(path = "models/Seal.glb#Scene0")]
    pub seal: Handle<Scene>,

    #[asset(path = "models/Squid.glb#Scene0")]
    pub squid: Handle<Scene>,

    #[asset(path = "models/StarFish.glb#Scene0")]
    pub starfish: Handle<Scene>,

    #[asset(path = "models/StingRay.glb#Scene0")]
    pub stingray: Handle<Scene>,

    #[asset(path = "models/TunaFish.glb#Scene0")]
    pub tuna_fish: Handle<Scene>,

    #[asset(path = "models/Turtle.glb#Scene0")]
    pub turtle: Handle<Scene>,

    #[asset(path = "models/Whale.glb#Scene0")]
    pub whale: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct FishAnimationCollection {
    #[asset(
        paths("models/BrownFish.glb#Animation0", "models/BrownFish.glb#Animation1"),
        collection(typed)
    )]
    pub brown_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/ClownFish.glb#Animation0", "models/ClownFish.glb#Animation1"),
        collection(typed)
    )]
    pub clown_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Crab.glb#Animation0", "models/Crab.glb#Animation1"),
        collection(typed)
    )]
    pub crab: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/DoryFish.glb#Animation0", "models/DoryFish.glb#Animation1"),
        collection(typed)
    )]
    pub dory_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Eel.glb#Animation0", "models/Eel.glb#Animation1"),
        collection(typed)
    )]
    pub eel: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Hammerhead.glb#Animation0", "models/Hammerhead.glb#Animation1"),
        collection(typed)
    )]
    pub hammerhead: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Lobster.glb#Animation0", "models/Lobster.glb#Animation1"),
        collection(typed)
    )]
    pub lobster: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Octopus.glb#Animation0", "models/Octopus.glb#Animation1"),
        collection(typed)
    )]
    pub octopus: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Penguin.glb#Animation0", "models/Penguin.glb#Animation1"),
        collection(typed)
    )]
    pub penguin: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Seal.glb#Animation0", "models/Seal.glb#Animation1"),
        collection(typed)
    )]
    pub seal: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Squid.glb#Animation0", "models/Squid.glb#Animation1"),
        collection(typed)
    )]
    pub squid: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/StarFish.glb#Animation0", "models/StarFish.glb#Animation1"),
        collection(typed)
    )]
    pub starfish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/StingRay.glb#Animation0", "models/StingRay.glb#Animation1"),
        collection(typed)
    )]
    pub stingray: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/TunaFish.glb#Animation0", "models/TunaFish.glb#Animation1"),
        collection(typed)
    )]
    pub tuna_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Turtle.glb#Animation0", "models/Turtle.glb#Animation1"),
        collection(typed)
    )]
    pub turtle: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Whale.glb#Animation0", "models/Whale.glb#Animation1"),
        collection(typed)
    )]
    pub whale: Vec<Handle<AnimationClip>>,
}

impl FishType {
    /// Gets the corresponding fish model for the given fish type
    pub fn model_from(&self, collection: &FishCollection) -> Handle<Scene> {
        match self {
            FishType::BrownFish => collection.brown_fish.clone(),
            FishType::ClownFish => collection.clown_fish.clone(),
            FishType::Crab => collection.crab.clone(),
            FishType::DoryFish => collection.dory_fish.clone(),
            FishType::Eel => collection.eel.clone(),
            FishType::Hammerhead => collection.hammerhead.clone(),
            FishType::Lobster => collection.lobster.clone(),
            FishType::Octopus => collection.octopus.clone(),
            FishType::Penguin => collection.penguin.clone(),
            FishType::Seal => collection.seal.clone(),
            FishType::Squid => collection.squid.clone(),
            FishType::StarFish => collection.starfish.clone(),
            FishType::StingRay => collection.stingray.clone(),
            FishType::TunaFish => collection.tuna_fish.clone(),
            FishType::Turtle => collection.turtle.clone(),
            FishType::Whale => collection.whale.clone(),
        }
    }

    // Gets the corresponding fish animation for the given fish type
    pub fn animations_from(&self, collection: &FishAnimationCollection) -> FishAnimations {
        match self {
            // TODO: busted
            FishType::BrownFish => FishAnimations {
                idle: collection.brown_fish[0].clone(),
                moving: Some(collection.brown_fish[1].clone()),
            },
            FishType::ClownFish => FishAnimations {
                idle: collection.clown_fish[0].clone(),
                moving: None,
            },
            FishType::Crab => FishAnimations {
                idle: collection.crab[0].clone(),
                moving: Some(collection.crab[1].clone()),
            },
            // TODO: busted
            FishType::DoryFish => FishAnimations {
                idle: collection.dory_fish[0].clone(),
                moving: Some(collection.dory_fish[1].clone()),
            },
            FishType::Eel => FishAnimations {
                idle: collection.eel[0].clone(),
                moving: Some(collection.eel[1].clone()),
            },
            FishType::Hammerhead => FishAnimations {
                idle: collection.hammerhead[0].clone(),
                moving: None,
            },
            FishType::Lobster => FishAnimations {
                idle: collection.lobster[0].clone(),
                moving: Some(collection.lobster[1].clone()),
            },
            FishType::Octopus => FishAnimations {
                idle: collection.octopus[1].clone(),
                moving: Some(collection.octopus[0].clone()),
            },
            FishType::Penguin => FishAnimations {
                idle: collection.penguin[0].clone(),
                moving: None,
            },
            FishType::Seal => FishAnimations {
                idle: collection.seal[0].clone(),
                moving: Some(collection.seal[1].clone()),
            },
            // This is flipped about the y axis
            FishType::Squid => FishAnimations {
                idle: collection.squid[0].clone(),
                moving: Some(collection.squid[1].clone()),
            },
            FishType::StarFish => FishAnimations {
                idle: collection.starfish[0].clone(),
                moving: None,
            },
            FishType::StingRay => FishAnimations {
                idle: collection.stingray[0].clone(),
                moving: None,
            },
            FishType::TunaFish => FishAnimations {
                idle: collection.tuna_fish[0].clone(),
                moving: None,
            },
            FishType::Turtle => FishAnimations {
                idle: collection.turtle[0].clone(),
                moving: Some(collection.turtle[1].clone()),
            },
            FishType::Whale => FishAnimations {
                idle: collection.whale[0].clone(),
                moving: None,
            },
        }
    }
}

pub struct FishAnimations {
    pub idle: Handle<AnimationClip>,

    pub moving: Option<Handle<AnimationClip>>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureCollection {
    #[asset(path = "textures/background.jpg")]
    pub background: Handle<Image>,
}
