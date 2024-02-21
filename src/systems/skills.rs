use crate::prelude::*;

#[system]
#[read_component(ActivateSkill)]
#[read_component(Stomp)]
#[read_component(Point)]
#[write_component(Skill)]
#[write_component(Health)]
pub fn use_skills(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut damage_to_inflict = Vec::<(Entity, i32)>::new();
    let mut used_skills = Vec::<Entity>::new();

    <(Entity, &ActivateSkill)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let character_ref = ecs.entry_ref(activate.used_by).unwrap();
            let character_pos = character_ref.get_component::<Point>().unwrap();
            if let Ok(s) = ecs.entry_ref(activate.skill) {
                let skill = s.get_component::<Skill>().unwrap();
                if skill.cooldown_cur == 0 {
                    if let Ok(stomp) = s.get_component::<Stomp>() {
                        <(Entity, &Point)>::query()
                            .iter(ecs)
                            .filter(|(_, pos)| {
                                (pos.x == character_pos.x - 1 && pos.y == character_pos.y)
                                    || (pos.x == character_pos.x + 1 && pos.y == character_pos.y)
                                    || (pos.x == character_pos.x && pos.y == character_pos.y - 1)
                                    || (pos.x == character_pos.x && pos.y == character_pos.y + 1)
                            })
                            .for_each(|(target, _)| {
                                damage_to_inflict.push((*target, stomp.damage));
                            });
                    }

                    used_skills.push(activate.skill);
                }
            }

            commands.remove(*entity);
        });

    for damage in damage_to_inflict.iter() {
        if let Ok(mut target) = ecs.entry_mut(damage.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current -= damage.1;

                if health.current < 1 {
                    commands.remove(damage.0);
                }
            }
        }
    }

    for entity in used_skills.iter() {
        if let Ok(mut s) = ecs.entry_mut(*entity) {
            let mut skill = s.get_component_mut::<Skill>().unwrap();
            skill.cooldown_cur = skill.cooldown_max;
        }
    }
}

#[system]
#[write_component(Skill)]
pub fn reduce_skill_cooldowns(ecs: &mut SubWorld) {
    <&mut Skill>::query()
        .iter_mut(ecs)
        .for_each(|skill| {
            if skill.cooldown_cur > 0 {
                skill.cooldown_cur -= 1;
            }
        });
}