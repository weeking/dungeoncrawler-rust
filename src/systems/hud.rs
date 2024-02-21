use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Skill)]
#[read_component(ExperienceLevel)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut exp_query = <&ExperienceLevel>::query().filter(component::<Player>());
    let player_exp = exp_query.iter(ecs).nth(0).unwrap();

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    y = 3;
    let mut skill_query = <(&Skill, &Name)>::query();
    skill_query
        .iter(ecs)
        .filter(|(skill, _)| skill.entity == player)
        .for_each(|(skill, name)| {
            draw_batch.print_right(
                Point::new(SCREEN_WIDTH * 2, y),
                format!(
                    "{} : {}/{}",
                    &name.0, &skill.cooldown_cur, &skill.cooldown_max
                ),
            );
            y += 1;
        });

    draw_batch.print_centered(1, "Explore the dungeon. Arrow keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health {}/{}", player_health.current, player_health.max,),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.bar_horizontal(
        Point::new(0, 1),
        SCREEN_WIDTH * 2,
        player_exp.experience,
        player_exp.level * 10,
        ColorPair::new(CYAN, BLACK),
    );
    draw_batch.print_color_centered(
        1,
        format!("Level {} (exp {}/{})", player_exp.level, player_exp.experience, player_exp.experience_max),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.submit(10000).expect("Batch error");
}
