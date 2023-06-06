#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::AnimationClip;
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

impl CoralCollection {
    /// Gets the corresponding coral model for the given coral type
    pub fn model_for(&self, coral_type: &CoralType) -> Handle<Scene> {
        match coral_type {
            CoralType::Coral => self.coral.clone(),
            CoralType::Coral1 => self.coral_1.clone(),
            CoralType::Coral2 => self.coral_2.clone(),
            CoralType::Coral3 => self.coral_3.clone(),
            CoralType::Coral4 => self.coral_4.clone(),
            CoralType::Coral5 => self.coral_5.clone(),
            CoralType::Coral6 => self.coral_6.clone(),
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

impl RockCollection {
    /// Gets the corresponding rock model for the given rock type
    pub fn model_for(&self, rock_type: &RockType) -> Handle<Scene> {
        match rock_type {
            RockType::Rock => self.rock.clone(),
            RockType::Rock1 => self.rock_1.clone(),
            RockType::Rock2 => self.rock_2.clone(),
            RockType::Rock3 => self.rock_3.clone(),
            RockType::Rock4 => self.rock_4.clone(),
            RockType::Rock5 => self.rock_5.clone(),
            RockType::Rock6 => self.rock_6.clone(),
            RockType::Rock7 => self.rock_7.clone(),
            RockType::Rock8 => self.rock_8.clone(),
            RockType::Rock9 => self.rock_9.clone(),
            RockType::Rock10 => self.rock_10.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum SeaweedType {
    // TODO: This is broken.
    // Seaweed,
    Seaweed1,
    Seaweed2,
}

#[derive(AssetCollection, Resource)]
pub struct SeaweedCollection {
    // #[asset(path = "models/SeaWeed1.glb#Scene0")]
    // seaweed: Handle<Scene>,
    #[asset(path = "models/Shells1.glb#Scene0")]
    seaweed_1: Handle<Scene>,

    #[asset(path = "models/Shells2.glb#Scene0")]
    seaweed_2: Handle<Scene>,
}

impl SeaweedCollection {
    /// Gets the corresponding seaweed model for the given seaweed type
    pub fn model_for(&self, seaweed_type: &SeaweedType) -> Handle<Scene> {
        match seaweed_type {
            // SeaweedType::Seaweed => self.seaweed.clone(),
            SeaweedType::Seaweed1 => self.seaweed_1.clone(),
            SeaweedType::Seaweed2 => self.seaweed_2.clone(),
        }
    }
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

impl ShellsCollection {
    /// Gets the corresponding shell model for the given shell type
    pub fn model_for(&self, shell_type: &ShellType) -> Handle<Scene> {
        match shell_type {
            ShellType::Shell => self.shells.clone(),
            ShellType::Shell1 => self.shells_1.clone(),
            ShellType::Shell2 => self.shells_2.clone(),
            ShellType::Shell3 => self.shells_3.clone(),
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
    brown_fish: Handle<Scene>,

    #[asset(path = "models/ClownFish.glb#Scene0")]
    clown_fish: Handle<Scene>,

    #[asset(path = "models/Crab.glb#Scene0")]
    crab: Handle<Scene>,

    #[asset(path = "models/DoryFish.glb#Scene0")]
    dory_fish: Handle<Scene>,

    #[asset(path = "models/Eel.glb#Scene0")]
    eel: Handle<Scene>,

    #[asset(path = "models/Hammerhead.glb#Scene0")]
    hammerhead: Handle<Scene>,

    #[asset(path = "models/Lobster.glb#Scene0")]
    lobster: Handle<Scene>,

    #[asset(path = "models/Octopus.glb#Scene0")]
    octopus: Handle<Scene>,

    #[asset(path = "models/Penguin.glb#Scene0")]
    penguin: Handle<Scene>,

    #[asset(path = "models/Seal.glb#Scene0")]
    seal: Handle<Scene>,

    #[asset(path = "models/Squid.glb#Scene0")]
    squid: Handle<Scene>,

    #[asset(path = "models/StarFish.glb#Scene0")]
    starfish: Handle<Scene>,

    #[asset(path = "models/StingRay.glb#Scene0")]
    stingray: Handle<Scene>,

    #[asset(path = "models/TunaFish.glb#Scene0")]
    tuna_fish: Handle<Scene>,

    #[asset(path = "models/Turtle.glb#Scene0")]
    turtle: Handle<Scene>,

    #[asset(path = "models/Whale.glb#Scene0")]
    whale: Handle<Scene>,
}

impl FishCollection {
    /// Gets the corresponding fish model for the given fish type
    pub fn model_for(&self, fish_type: &FishType) -> Handle<Scene> {
        match fish_type {
            FishType::BrownFish => self.brown_fish.clone(),
            FishType::ClownFish => self.clown_fish.clone(),
            FishType::Crab => self.crab.clone(),
            FishType::DoryFish => self.dory_fish.clone(),
            FishType::Eel => self.eel.clone(),
            FishType::Hammerhead => self.hammerhead.clone(),
            FishType::Lobster => self.lobster.clone(),
            FishType::Octopus => self.octopus.clone(),
            FishType::Penguin => self.penguin.clone(),
            FishType::Seal => self.seal.clone(),
            FishType::Squid => self.squid.clone(),
            FishType::StarFish => self.starfish.clone(),
            FishType::StingRay => self.stingray.clone(),
            FishType::TunaFish => self.tuna_fish.clone(),
            FishType::Turtle => self.turtle.clone(),
            FishType::Whale => self.whale.clone(),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct FishAnimationCollection {
    #[asset(
        paths("models/BrownFish.glb#Animation0", "models/BrownFish.glb#Animation1"),
        collection(typed)
    )]
    brown_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/ClownFish.glb#Animation0", "models/ClownFish.glb#Animation1"),
        collection(typed)
    )]
    clown_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Crab.glb#Animation0", "models/Crab.glb#Animation1"),
        collection(typed)
    )]
    crab: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/DoryFish.glb#Animation0", "models/DoryFish.glb#Animation1"),
        collection(typed)
    )]
    dory_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Eel.glb#Animation0", "models/Eel.glb#Animation1"),
        collection(typed)
    )]
    eel: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Hammerhead.glb#Animation0", "models/Hammerhead.glb#Animation1"),
        collection(typed)
    )]
    hammerhead: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Lobster.glb#Animation0", "models/Lobster.glb#Animation1"),
        collection(typed)
    )]
    lobster: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Octopus.glb#Animation0", "models/Octopus.glb#Animation1"),
        collection(typed)
    )]
    octopus: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Penguin.glb#Animation0", "models/Penguin.glb#Animation1"),
        collection(typed)
    )]
    penguin: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Seal.glb#Animation0", "models/Seal.glb#Animation1"),
        collection(typed)
    )]
    seal: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Squid.glb#Animation0", "models/Squid.glb#Animation1"),
        collection(typed)
    )]
    squid: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/StarFish.glb#Animation0", "models/StarFish.glb#Animation1"),
        collection(typed)
    )]
    starfish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/StingRay.glb#Animation0", "models/StingRay.glb#Animation1"),
        collection(typed)
    )]
    stingray: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/TunaFish.glb#Animation0", "models/TunaFish.glb#Animation1"),
        collection(typed)
    )]
    tuna_fish: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Turtle.glb#Animation0", "models/Turtle.glb#Animation1"),
        collection(typed)
    )]
    turtle: Vec<Handle<AnimationClip>>,

    #[asset(
        paths("models/Whale.glb#Animation0", "models/Whale.glb#Animation1"),
        collection(typed)
    )]
    whale: Vec<Handle<AnimationClip>>,
}

pub struct FishAnimations {
    pub idle: Handle<AnimationClip>,

    pub moving: Option<Handle<AnimationClip>>,
}

impl FishAnimationCollection {
    // Gets the corresponding fish animation for the given fish type
    pub fn animations_for(&self, fish_type: &FishType) -> FishAnimations {
        match fish_type {
            // TODO: busted
            FishType::BrownFish => FishAnimations {
                idle: self.brown_fish[0].clone(),
                moving: Some(self.brown_fish[1].clone()),
            },
            FishType::ClownFish => FishAnimations {
                idle: self.clown_fish[0].clone(),
                moving: None,
            },
            FishType::Crab => FishAnimations {
                idle: self.crab[0].clone(),
                moving: Some(self.crab[1].clone()),
            },
            // TODO: busted
            FishType::DoryFish => FishAnimations {
                idle: self.dory_fish[0].clone(),
                moving: Some(self.dory_fish[1].clone()),
            },
            FishType::Eel => FishAnimations {
                idle: self.eel[0].clone(),
                moving: Some(self.eel[1].clone()),
            },
            FishType::Hammerhead => FishAnimations {
                idle: self.hammerhead[0].clone(),
                moving: None,
            },
            FishType::Lobster => FishAnimations {
                idle: self.lobster[0].clone(),
                moving: Some(self.lobster[1].clone()),
            },
            FishType::Octopus => FishAnimations {
                idle: self.octopus[1].clone(),
                moving: Some(self.octopus[0].clone()),
            },
            FishType::Penguin => FishAnimations {
                idle: self.penguin[0].clone(),
                moving: None,
            },
            FishType::Seal => FishAnimations {
                idle: self.seal[0].clone(),
                moving: Some(self.seal[1].clone()),
            },
            // This is flipped about the y axis
            FishType::Squid => FishAnimations {
                idle: self.squid[0].clone(),
                moving: Some(self.squid[1].clone()),
            },
            FishType::StarFish => FishAnimations {
                idle: self.starfish[0].clone(),
                moving: None,
            },
            FishType::StingRay => FishAnimations {
                idle: self.stingray[0].clone(),
                moving: None,
            },
            FishType::TunaFish => FishAnimations {
                idle: self.tuna_fish[0].clone(),
                moving: None,
            },
            FishType::Turtle => FishAnimations {
                idle: self.turtle[0].clone(),
                moving: Some(self.turtle[1].clone()),
            },
            FishType::Whale => FishAnimations {
                idle: self.whale[0].clone(),
                moving: None,
            },
        }
    }
}
