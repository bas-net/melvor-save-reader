use serde_json::{Map, Value};

use super::character_decoder::CharacterDecoder;

pub trait PlayerDecoder: CharacterDecoder {
    fn decode_player(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert("character".into(), r.decode_character());

        // Melee
        if r.read_bool() {
            map.insert(
                "melee_style".into(),
                r.read_namespaced_object().into(),
            );
        }
        // Ranged
        if r.read_bool() {
            map.insert(
                "ranged_style".into(),
                r.read_namespaced_object().into(),
            );
        }
        // Magic
        if r.read_bool() {
            map.insert(
                "magic_style".into(),
                r.read_namespaced_object().into(),
            );
        }
        map.insert("prayer_points".into(), r.read_uint32().into());
        map.insert("selected_equipment_set".into(), r.read_uint16().into());

        // Equipment sets
        map.insert(
            "equipment_sets".into(),
            r.read_vector(|r| -> Value {
                let mut map = Map::new();
                // Equipment
                map.insert(
                    "equipment".into(),
                    r.read_vector(|r| -> Value {
                        let mut map = Map::new();
                        map.insert(
                            "slot".into(),
                            r.read_namespaced_object().into(),
                        );
                        if r.read_bool() {
                            map.insert(
                                "item".into(),
                                r.read_namespaced_object().into(),
                            );
                            map.insert(
                                "quantity".into(),
                                r.read_uint32().into(),
                            );
                        }
                        map.insert(
                            "quick_equip_items".into(),
                            r.read_vector(|r| -> Value {
                                r.read_namespaced_object().into()
                            })
                            .into(),
                        );

                        map.into()
                    })
                    .into(),
                );

                // Spell Selection
                if r.read_bool() {
                    map.insert(
                        "spell".into(),
                        r.read_namespaced_object().into(),
                    );
                }
                if r.read_bool() {
                    map.insert(
                        "aurora".into(),
                        r.read_namespaced_object().into(),
                    );
                }
                if r.read_bool() {
                    map.insert(
                        "curse".into(),
                        r.read_namespaced_object().into(),
                    );
                }

                // Prayer selection (set)
                map.insert(
                    "prayers".into(),
                    r.read_set(|r| -> Value {
                        r.read_namespaced_object().into()
                    })
                    .into(),
                );

                map.into()
            })
            .into(),
        );

        // player.food
        map.insert("food".into(), {
            let mut map = Map::new();
            map.insert("selected_slot".into(), r.read_uint32().into());
            map.insert("max_slots".into(), r.read_uint32().into());

            map.insert(
                "slots".into(),
                r.read_vector(|r| -> Value {
                    let mut map = Map::new();
                    map.insert(
                        "item".into(),
                        r.read_namespaced_object().into(),
                    );
                    map.insert("quantity".into(), r.read_uint32().into());
                    map.into()
                })
                .into(),
            );

            map.into()
        });

        // player.timers.summon
        map.insert("summon_timer".into(), r.decode_timer());

        map.insert("soul_points".into(), r.read_uint32().into());
        map.insert("unholy_prayer_multiplier".into(), r.read_uint8().into());

        map.into()
    }
}
