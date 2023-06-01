// credit: https://github.com/devjobe/ambient_third_person_character/blob/main/src/anim.rs

#![allow(unused)]

use ambient_api::{
    asset,
    entity::{self, AnimationAction, AnimationController},
    prelude::{EntityId, Vec3},
};

pub const MAX_SPEED: f32 = 6.0;
pub const WALK_SPEED: f32 = 2.0;
pub const INV_ACCELERATION_TIME: f32 = 10.0;
pub const GRAVITY: f32 = 9.8;
pub const JUMP_HEIGHT: f32 = 2.0;
pub const LOCAL_FORWARD: Vec3 = Vec3::X;
pub const LOCAL_RIGHT: Vec3 = Vec3::Y;
pub const LOCAL_UP: Vec3 = Vec3::Z;
pub const WORLD_DOWN: Vec3 = Vec3::NEG_Z;
pub const TERMINAL_SPEED: f32 = 100.0;

pub struct AnimationAssets {
    pub idle: String,
    pub walking: String,
    pub zombie_walk: String,
    pub zombie_reaction: String,
    pub running: String,
    pub jump: String,
    pub falling_idle: String,
    pub hit: String,
    pub fire: String,
    pub death: String,
}

#[derive(Default, Debug)]
pub struct Blend {
    pub idle: f32,
    pub walking: f32,
    pub running: f32,
    pub jump: f32,
    pub falling_idle: f32,
    pub hit: f32,
    pub fire: f32,
    pub death: f32,
}

impl AnimationAssets {
    pub fn new() -> Self {
        AnimationAssets {
            idle: asset::url("assets/xbot/Rifle Aiming Idle.fbx/animations/mixamo.com.anim").unwrap(),
            walking: asset::url("assets/xbot/Rifle Walking.fbx/animations/mixamo.com.anim").unwrap(),
            zombie_walk: asset::url("assets/xbot/Zombie Walk.fbx/animations/mixamo.com.anim").unwrap(),
            zombie_reaction: asset::url("assets/xbot/Zombie Reaction Hit.fbx/animations/mixamo.com.anim").unwrap(),
            running: asset::url("assets/xbot/Rifle Run.fbx/animations/mixamo.com.anim").unwrap(),
            jump: asset::url("assets/xbot/Rifle Jump.fbx/animations/mixamo.com.anim").unwrap(),
            falling_idle: asset::url("assets/xbot/Rifle Aiming Idle.fbx/animations/mixamo.com.anim").unwrap(),
            hit: asset::url("assets/xbot/Hit Reaction.fbx/animations/mixamo.com.anim").unwrap(),
            fire: asset::url("assets/xbot/Rifle Firing.fbx/animations/mixamo.com.anim").unwrap(),
            death: asset::url("assets/xbot/Rifle Death.fbx/animations/mixamo.com.anim").unwrap(),
        }
    }

    pub fn set_fire (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.fire,
                looping: false,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_walk (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.walking,
                looping: true,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_running (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.running,
                looping: true,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_hit (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.hit,
                looping: false,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_zombie_walk (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.zombie_walk,
                looping: true,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_zombie_reaction (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.zombie_reaction,
                looping: false,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_idle (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.idle,
                looping: true,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_death (&self, entity: EntityId) {
        let actions = [
            AnimationAction {
                clip_url: &self.death,
                looping: false,
                weight: 1.0,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_controller(&self, entity: EntityId, blend: Blend) {
        let actions = [
            AnimationAction {
                clip_url: &self.idle,
                looping: true,
                weight: blend.idle,
            },
            AnimationAction {
                clip_url: &self.walking,
                looping: true,
                weight: blend.walking,
            },
            AnimationAction {
                clip_url: &self.running,
                looping: true,
                weight: blend.running,
            },
            AnimationAction {
                clip_url: &self.jump,
                looping: true,
                weight: blend.jump,
            },
            AnimationAction {
                clip_url: &self.falling_idle,
                looping: true,
                weight: blend.falling_idle,
            },
            AnimationAction {
                clip_url: &self.hit,
                looping: true,
                weight: blend.hit,
            },
            AnimationAction {
                clip_url: &self.fire,
                looping: true,
                weight: blend.fire,
            },
            AnimationAction {
                clip_url: &self.death,
                looping: true,
                weight: blend.death,
            },
        ];

        entity::set_animation_controller(
            entity,
            AnimationController {
                actions: &actions,
                apply_base_pose: false,
            },
        );
    }

    pub fn set_blend(entity: EntityId, blend: Blend) {
        entity::set_animation_blend(
            entity,
            &[
                blend.idle,
                blend.walking,
                blend.running,
                blend.jump,
                blend.falling_idle,
                blend.hit,
                blend.fire,
                blend.death,
            ],
            &[],
            false
        );
    }
}