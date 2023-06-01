use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::{children, parent},
        prefab::prefab_from_url,
        physics::{
            dynamic, visualizing,
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

use components::*;
use std::f32::consts::{PI, TAU};

mod anim;
mod zombie;

#[main]
pub async fn main() {

    zombie::make_zombies(3);

    let animations = anim::AnimationAssets::new();

    // let cam = Entity::new()
    //     .with_merge(make_perspective_infinite_reverse_camera())
    //     .with(aspect_ratio_from_window(), EntityId::resources())
    //     .with_default(main_scene())
    //     .with(translation(), vec3(10.0, 0.0, 10.0)* 1.0)
    //     .with(lookat_target(), vec3(0., 0., 0.))
    //     .spawn();

    spawn_query(player()).bind(move |players| {
        for (id, _) in players {
            let cam = Entity::new()
                .with_merge(make_perspective_infinite_reverse_camera())
                .with(aspect_ratio_from_window(), EntityId::resources())
                .with_default(main_scene())
                // .with(user_id(), uid.clone())
                .with(translation(), vec3(-0.0, 5.0, 3.0))
                .with(parent(), id)
                .with_default(local_to_parent())
                .with_default(local_to_world())
                .with(rotation(), Quat::from_rotation_x(PI/2.0))
                .spawn();

            let model = Entity::new()
            .with_merge(make_transformable())
            .with(prefab_from_url(), asset::url("assets/model/Y Bot.fbx").unwrap())
            .with(rotation(), Quat::from_rotation_z(-3.14159265359))
            .with_default(local_to_parent())
            .with(parent(), id)
            .spawn();

            entity::add_components(
                id,
                Entity::new()
                .with_merge(make_transformable())
                .with(character_controller_height(), 2.0)
                .with(character_controller_radius(), 0.5)
                .with_default(physics_controlled())
                .with(children(), vec![model, cam])
                .with(player_head_ref(), cam)
                .with(components::model_ref(), model)
                .with(player_pitch(), 0.0)
                .with(player_yaw(), 0.0)
                .with(translation(), vec3(0., 0., 5.))
                .with_default(local_to_world())
            );
            let mut blend = anim::Blend::default();
            blend.idle = 1.0;
            animations.set_controller(model, blend);
        }
    });
    
    Entity::new()
        .with_merge(make_transformable())
        // .with(prefab_from_url(), asset::url("assets/map/ground.glb").unwrap())
        .with_default(quad())
        .with_default(plane_collider())
        .with(translation(), vec3(0., 0., -10.))
        .with(scale(), Vec3::ONE*200.0)
        .spawn();

    messages::Input::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };

        entity::add_component(player_id, components::player_movement_direction(), msg.direction);

        let yaw = entity::mutate_component(player_id, components::player_yaw(), |yaw| {
            *yaw = (*yaw + msg.mouse_delta.x * 0.01) % TAU;
        })
        .unwrap_or_default();
        let pitch = entity::mutate_component(player_id, player_pitch(), |pitch| {
            *pitch = (*pitch + msg.mouse_delta.y * 0.01).clamp(-PI / 3., PI / 3.);
        })
        .unwrap_or_default();
        entity::set_component(player_id, rotation(), Quat::from_rotation_z(yaw));
        if let Some(head_id) = entity::get_component(player_id, player_head_ref()) {
            entity::set_component(head_id, rotation(), Quat::from_rotation_x(PI / 2. + pitch));
        }
    });

    messages::Ray::subscribe(move |_source, msg| {
        let result = physics::raycast_first(msg.ray_origin, msg.ray_dir);
        if let Some(hit) = result {     
            if entity::has_component(hit.entity, zombie_model_ref()) && msg.type_action == 0 {
                println!("hit zombie");
                entity::mutate_component(hit.entity, zombie_health(), |x| {
                    if *x <= 0 {
                        return;
                    }
                    if *x - 10 < 0 {
                        run_async(async move {
                            println!("zombie dead");
                            
                        });
                    } else {
                        *x-=10;
                    }
                });
            }
        }
    });

    // query the player's component each frame
    // determine the direction of the player's movement
    // calculate animations blend
    query((player(), player_movement_direction(), rotation())).each_frame(move |players| {
        for (player_id, (_, direction, rot)) in players {
            // println!("direction: {:?}", direction);
            let speed = 0.1;
            let displace = rot * (direction.normalize_or_zero() * speed).extend(-0.1);
            
            if let Some(model) = entity::get_component::<EntityId>(player_id, components::model_ref()) {
                // println!("model: {:?}", model);
                let mut blend = anim::Blend::default();
                
                if direction == Vec2::ZERO {
                    blend.idle = 1.0;
                } else {
                    if direction.y == -1.0 {
                        blend.forward = 1.0;
                    } else if direction.y == 1.0 {
                        blend.backward = 1.0;
                    }
                    if direction.x == -1.0 {
                        blend.left = 1.0;
                    } else if direction.x == 1.0 {
                        blend.right = 1.0;
                    }
                }
                // println!("blend: {:?}", blend);
                // animations.set_controller(model, blend);
                anim::set_blend(model, blend);
                // let collision = physics::move_character(model, displace, 0.01, frametime());
                // println!("collision: {} {} {}", collision.up, collision.down, collision.side);
            }
            let collision = physics::move_character(
                player_id, displace, 0.01, frametime());
            // println!("collision: up {} down {} side {}", collision.up, collision.down, collision.side);
        }
    });
}
