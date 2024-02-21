use crate::prelude::*;

#[system]
#[read_component(AddExperience)]
#[read_component(Player)]
#[read_component(ProvidesExperience)]
#[write_component(ExperienceLevel)]
#[write_component(Health)]
pub fn add_experience(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut exp_to_add = 0;
    <(Entity, &AddExperience)>::query()
        .iter(ecs)
        .for_each(|(entity, add_exp)| {
            let exp = <(Entity, &ProvidesExperience)>::query()
                .iter(ecs)
                .filter(|(entity, _)| **entity == add_exp.0)
                .find_map(|(_, exp)| Some(exp.0)).unwrap();
            exp_to_add += exp;
            commands.remove(*entity);
            commands.remove(add_exp.0);
        });
    
    <(Entity, &mut ExperienceLevel, &mut Health)>::query().iter_mut(ecs)
        .for_each(|(_, exp, health)| {
            exp.experience += exp_to_add;

            if exp.experience >= exp.experience_max {
                exp.experience = exp.experience % exp.experience_max;
                exp.level += 1;
                exp.experience_max = exp.level * 10;

                health.max += exp.level;
            }
        });
}
