use ambient_api::prelude::*;

#[element_component]
pub fn App(hooks: &mut Hooks) -> Element {

    // let (players, set_players) = hooks.use_state(vec![]);

    // change_query((anim_character(), health(), components::death_count())).track_change((health(), components::death_count())).bind(move |players| {
    //     let mut v = vec![];
    //     // this id is actually the model id, thus model health :P
    //     for (id, (_, health, death_count)) in players {
    //         println!("changes detected in players! {:?} {:?}", health, death_count);
    //         v.push((id, health, death_count));
    //     }
    //     set_players(v);
    // });

    let window_size = entity::get_component(entity::resources(), window_logical_size()).unwrap();
    let mut center_x = window_size.x as f32 / 2.;
    let mut center_y = window_size.y as f32 / 2.;

    let size_info = hooks.use_query(window_logical_size());

    // let mut  center_x; // = window_size.x as f32 / 2.;
    // let mut center_y; // = window_size.y as f32 / 2.;
    for (resource_id, xy) in size_info {
        println!("window size: {:?} {:?}", resource_id, xy);
        center_x = xy.x as f32 / 2.;
        center_y = xy.y as f32 / 2.;
    }
    
    // let window_size = entity::get_component(entity::resources(), window_logical_size()).unwrap();
    // let center_x = window_size.x as f32 / 2.;
    // let center_y = window_size.y as f32 / 2.;
    Group::el([
        // FocusRoot::el([FlowColumn::el(
        //     [
        //         Text::el("[wsad] => move; [space] => jump; [shift] => run;"),
        //         Text::el("[mouse move] => look around; [click] => shoot;"),
        //         Text::el("dead players will respawn in 2 seconds"),
        //     ]
        //     // players.iter().map(|(id, health, death_count)| {
        //     //     if *health == 0 {
        //     //         FlowRow::el([
        //     //             Text::el(format!("Player {}", id)),
        //     //             Text::el(format!("Hit out! Wait for respawn!")),
        //     //             Text::el(format!("Death count: {}", death_count)),
        //     //         ])
        //     //     } else {
        //     //         FlowRow::el([
        //     //             Text::el(format!("Player {}", id)),
        //     //             Text::el(format!("Health: {:.2}", health)),
        //     //             Text::el(format!("Death count: {}", death_count)),
        //     //         ])
        //     //     }
        //     //     .with(space_between_items(), STREET)
        //     // })
        // )
        // .with_background(vec4(0., 0., 0., 0.9))
        // .with_padding_even(10.)]),
        Line.el()
            .with(line_from(), vec3(center_x - 10., center_y, 0.))
            .with(line_to(), vec3(center_x + 10., center_y, 0.))
            .with(line_width(), 2.)
            .with(background_color(), vec4(1., 1., 1., 1.)),
        Line.el()
            .with(line_from(), vec3(center_x, center_y - 10., 0.))
            .with(line_to(), vec3(center_x, center_y + 10., 0.))
            .with(line_width(), 2.)
            .with(background_color(), vec4(1., 1., 1., 1.)),
    ])
}