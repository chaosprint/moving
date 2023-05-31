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
    pub forward: String,
    pub backward: String,
    pub right: String,
    pub left: String,
}

#[derive(Default, Debug)]
pub struct Blend {
    pub idle: f32,
    pub forward: f32,
    pub backward: f32,
    pub right: f32,
    pub left: f32,
}

impl AnimationAssets {
    pub fn new() -> Self {
        AnimationAssets {
            idle: asset::url("assets/anim/Rifle Aiming Idle.fbx/animations/mixamo.com.anim").unwrap(),
            forward: asset::url("assets/anim/Rifle Walking Forward.fbx/animations/mixamo.com.anim").unwrap(),
            backward: asset::url("assets/anim/Rifle Walking Backward.fbx/animations/mixamo.com.anim").unwrap(),
            left: asset::url("assets/anim/Rifle Walk Left.fbx/animations/mixamo.com.anim").unwrap(),
            right: asset::url("assets/anim/Rifle Walk Right.fbx/animations/mixamo.com.anim").unwrap(),
        }
    }

    pub fn set_controller(&self, entity: EntityId, blend: Blend) {
        let actions = [
            AnimationAction {
                clip_url: &self.idle,
                looping: true,
                weight: blend.idle,
            },
            AnimationAction {
                clip_url: &self.forward,
                looping: true,
                weight: blend.forward,
            },
            AnimationAction {
                clip_url: &self.backward,
                looping: true,
                weight: blend.backward,
            },
            AnimationAction {
                clip_url: &self.left,
                looping: true,
                weight: blend.left,
            },
            AnimationAction {
                clip_url: &self.right,
                looping: true,
                weight: blend.right,
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
}

pub fn set_blend(entity: EntityId, blend: Blend) {
    entity::set_animation_blend(
        entity,
        &[
            blend.idle,
            blend.forward,
            blend.backward,
            blend.left,
            blend.right,
        ],
        &[],
        false
    );
}