use crate::prelude::*;

mod template;

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = template::Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let player = ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
            layer: 2,
        },
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
        Damage(1),
        ExperienceLevel {
            level: 1,
            experience: 0,
            experience_max: 10
        }
    ));

    ecs.push((
        Skill {
            entity: player,
            cooldown_max: 3,
            cooldown_cur: 0,
        },
        Name("Stomp".to_string()),
        Stomp { damage: 2 },
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
            layer: 1
        },
        Name("Amulet of Yala".to_string()),
    ));
}
