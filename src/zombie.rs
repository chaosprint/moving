use crate::components;

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children, parent},
        prefab::prefab_from_url,
        physics::{
            dynamic,
            character_controller_height, character_controller_radius, physics_controlled,
            plane_collider, sphere_collider, linear_velocity, cube_collider,
        },
        player::{player, user_id},
        primitives::{cube, quad},
        rendering::color,
        transform::{local_to_parent, rotation, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_sphere, make_transformable},
    prelude::*,
};

use components::{player_head_ref, player_movement_direction, player_pitch, player_yaw};
use std::f32::consts::{PI, TAU};

pub fn make_zombies(amount: usize) {
    let chars = vec![
        asset::url("assets/model/Zombiegirl W Kurniawan.fbx").unwrap(),
        asset::url("assets/model/copzombie_l_actisdato.fbx").unwrap(),
        asset::url("assets/model/Yaku J Ignite.fbx").unwrap(),
    ];

    run_async(async move {
        for i in 0..amount {
            let zombie = Entity::new()
            .spawn();
            
            let model = make_transformable()
            .with(prefab_from_url(), chars[i%3].clone())
            .with(parent(), zombie)
            .with_default(local_to_parent())
            .with(rotation(), Quat::from_rotation_z(-3.14159265359/2.0))
            // .with_default(local_to_world())
            .spawn();
        
            entity::add_components(
                zombie,
                make_transformable()
                .with(character_controller_height(), 2.)
                .with(character_controller_radius(), 0.5)
                .with(translation(), vec3(100.0*random::<f32>(), 100.0*random::<f32>(), 5.0))
                .with(children(), vec![model])
                .with_default(local_to_world())
                .with_default(physics_controlled())
                .with_default(components::is_zombie())
            );
            let actions = [
                entity::AnimationAction {
                    clip_url: &asset::url("assets/anim/Zombie Run.fbx/animations/mixamo.com.anim").unwrap(),
                    looping: true,
                    weight: 1.0,
                },
            ];

            entity::set_animation_controller(
                model,
                entity::AnimationController {
                    actions: &actions,
                    apply_base_pose: false,
                },
            );
            sleep(random::<f32>()).await;
        }
    });

    let player_query = query(translation()). requires(player()).build();
    query((translation(), components::is_zombie())).each_frame(move |zombies|{
        for (zombie, (pos, _)) in zombies {

            let players: Vec<(EntityId, Vec3)> = player_query.evaluate();
            let zombie_pos = vec2(pos.x, pos.y);

            let mut min_distance = std::f32::MAX;
            let mut nearest_player_pos: Option<Vec2> = None;

            for (player, pos) in players {
                // println!("player pos {:?}", pos);
                let player_pos = vec2(pos.x, pos.y);
                let distance = (zombie_pos - player_pos).length();
                if distance < min_distance {
                    min_distance = distance;
                    nearest_player_pos = Some(player_pos);
                }
            }
            
            
            if let Some(nearest_player_pos) = nearest_player_pos {
                let displace = nearest_player_pos - zombie_pos; // Here's your displacement vector
                let zb_speed = 0.03;
                // If you want the zombie to move at a constant speed regardless of distance to the player,
                // you may want to normalize the displacement vector before feeding it to `move_character`
                let displace = displace.normalize_or_zero() * zb_speed; // normalize to get a unit vector
                
                let angle = displace.y.atan2(displace.x);
                let rot = Quat::from_rotation_z(angle);
                let collision = physics::move_character(
                    zombie,
                    vec3(displace.x, displace.y, -0.1),
                    0.01,
                    frametime()
                );
                entity::set_component(zombie, rotation(), rot);
                // println!("collision: {} {} {}", collision.up, collision.down, collision.side);
            }
        }
    });
}