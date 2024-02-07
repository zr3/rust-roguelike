use crate::{
    components::{DefenseBonus, Equipped, HungerClock, HungerState, MeleePowerBonus, Position},
    gamelog::LogEntry,
    particle_system::ParticleBuilder,
};

use super::{gamelog::GameLog, CombatStats, Name, SufferDamage, WantsToMelee};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, MeleePowerBonus>,
        ReadStorage<'a, DefenseBonus>,
        ReadStorage<'a, Equipped>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, HungerClock>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut log,
            mut wants_melee,
            names,
            combat_stats,
            mut inflict_damage,
            melee_power_bonuses,
            defense_bonuses,
            equipped,
            mut particle_builder,
            positions,
            hunger_clock,
        ) = data;

        for (entity, wants_melee, name, stats) in
            (&entities, &wants_melee, &names, &combat_stats).join()
        {
            if stats.hp > 0 {
                let mut offensive_bonus = 0;
                for (_item_entity, power_bonus, equipped_by) in
                    (&entities, &melee_power_bonuses, &equipped).join()
                {
                    if equipped_by.owner == entity {
                        offensive_bonus += power_bonus.power;
                    }
                }
                let hc = hunger_clock.get(entity);
                if let Some(hc) = hc {
                    if hc.state == HungerState::Full {
                        offensive_bonus += 1;
                    }
                }

                let target_stats = combat_stats.get(wants_melee.target).expect(
                    "WantsToMelee should only exist on entities that also have CombatStats",
                );
                if target_stats.hp > 0 {
                    let target_name = names
                        .get(wants_melee.target)
                        .expect("WantsToMelee should only exist on entities that also have Name");

                    let mut defensive_bonus = 0;
                    for (_item_entity, defense_bonus, equipped_by) in
                        (&entities, &defense_bonuses, &equipped).join()
                    {
                        if equipped_by.owner == wants_melee.target {
                            defensive_bonus += defense_bonus.defense;
                        }
                    }

                    let pos = positions.get(wants_melee.target);
                    if let Some(pos) = pos {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::TOMATO),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('‼'),
                            150.0,
                        );
                    }

                    let damage = i32::max(
                        0,
                        (stats.power + offensive_bonus) - (target_stats.defense + defensive_bonus),
                    );
                    if damage == 0 {
                        log.log(LogEntry::Action {
                            subject: format!("{}", &name.name),
                            verb: format!("is not able to hurt"),
                            object: format!("{}", &target_name.name),
                            suffix: format!("."),
                        });
                    } else {
                        log.log(LogEntry::Action {
                            subject: format!("{}", &name.name),
                            verb: format!("hit"),
                            object: format!("{}", &target_name.name),
                            suffix: format!("for {} HP!", damage),
                        });
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
