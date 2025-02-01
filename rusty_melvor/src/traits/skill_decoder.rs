use serde_json::{Map, Value};

use super::{read::DataReaders, timer_decoder::TimerDecoder};

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait SkillDecoder: DataReaders + TimerDecoder {
    fn decode_skills(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        let number_of_skills = r.read_uint32();
        println!("Number of skills: {}", number_of_skills);
        for _ in 0..number_of_skills {
            let skill = r.read_namespaced_object();
            let skill_str = match skill.text_id {
                Some(text_id) => text_id,
                _ => skill.id.to_string(),
            };
            println!("Skill: {}", skill_str);

            let skill_data_size = r.read_uint32();

            match skill_str.as_str() {
                "melvorD:Attack" => {
                    madd!(map, "attack", r.decode_skill());
                }
                "melvorD:Strength" => {
                    madd!(map, "strength", r.decode_skill());
                }
                "melvorD:Defence" => {
                    madd!(map, "defense", r.decode_skill());
                }
                "melvorD:Hitpoints" => {
                    madd!(map, "hitpoints", r.decode_skill());
                }
                "melvorD:Ranged" => {
                    madd!(map, "ranged", r.decode_skill());
                }
                "melvorD:Magic" => {
                    madd!(map, "magic", r.decode_alt_magic());
                }
                "melvorD:Prayer" => {
                    madd!(map, "prayer", r.decode_skill());
                }
                "melvorD:Slayer" => {
                    madd!(map, "slayer", r.decode_skill());
                }
                "melvorD:Woodcutting" => {
                    madd!(map, "woodcutting", r.decode_woodcutting());
                }
                "melvorD:Fishing" => {
                    madd!(map, "fishing", r.decode_fishing());
                }
                "melvorD:Firemaking" => {
                    madd!(map, "firemaking", r.decode_firemaking());
                }
                _ => {
                    println!("Unknown skill: {}", skill_str);
                    r.skip(skill_data_size as usize);
                    break;
                }
            }

            // break;
        }

        map.into()
    }

    fn decode_skill(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "xp", r.read_float64());
        madd!(map, "unlocked", r.read_bool());
        madd!(
            map,
            "realms",
            r.read_value_map_key(
                |r| r.read_namespaced_object().as_name(),
                |r, _| {
                    let mut map = Map::new();
                    madd!(
                        map,
                        "found_relics",
                        r.read_value_map_key(
                            |r| r.read_namespaced_object().as_name(),
                            |r, _| r.read_uint8().into()
                        )
                    );

                    map.into()
                }
            )
        );

        madd!(map, "current_level_cap", r.read_uint16());
        madd!(map, "current_abyssal_level_cap", r.read_uint16());

        madd!(
            map,
            "skill_trees",
            r.read_vector(|r| -> Value {
                let mut map = Map::new();

                madd!(map, "skill_tree_name", r.read_namespaced_object());

                madd!(map, "skill_tree", {
                    let mut map = Map::new();
                    madd!(
                        map,
                        "nodes",
                        r.read_vector(|r| -> Value {
                            let mut map = Map::new();
                            madd!(map, "node", r.read_namespaced_object());
                            madd!(map, "unlocked", r.read_bool());
                            map.into()
                        })
                    );
                    madd!(map, "points", r.read_uint8());

                    map
                });

                map.into()
            },)
        );

        madd!(map, "abyssal_xp", r.read_float64());

        madd!(map, "realm", r.read_namespaced_object());

        map.into()
    }

    fn decode_skill_with_mastery(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "skill", r.decode_skill());

        madd!(
            map,
            "action_mastery",
            r.read_value_map_key(
                |r| r.read_namespaced_object().as_name(),
                |r, _| {
                    let mut map = Map::new();
                    madd!(map, "xp", r.read_float64());
                    map.into()
                }
            )
        );

        madd!(
            map,
            "mastery_pool_xp",
            r.read_sparse_numeric_map(|r| r
                .read_namespaced_object()
                .as_name())
        );

        map.into()
    }

    fn decode_gathering_skill(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "skill_with_mastery", r.decode_skill_with_mastery());

        madd!(map, "is_active", r.read_bool());
        madd!(map, "action_timer", r.decode_timer());

        map.into()
    }

    fn decode_alt_magic(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "gathering_skill", r.decode_gathering_skill());

        if r.read_bool() {
            madd!(map, "spell", r.read_namespaced_object());
        }
        if r.read_bool() {
            madd!(map, "conversion_item", r.read_namespaced_object());
        }
        if r.read_bool() {
            madd!(map, "selected_recipe", r.read_namespaced_object());
        }

        map.into()
    }

    fn decode_woodcutting(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "gathering_skill", r.decode_gathering_skill());

        madd!(
            map,
            "active_trees",
            r.read_set(|r| r.read_namespaced_object())
        );

        map.into()
    }

    fn decode_fishing(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "gathering_skill", r.decode_gathering_skill());

        madd!(map, "secret_area_unlocked", r.read_bool());

        // TODO: unsafe
        let is_active = map
            .get("gathering_skill")
            .unwrap()
            .get("is_active")
            .unwrap()
            .as_bool()
            .unwrap();
        if is_active {
            madd!(map, "area", r.read_namespaced_object());
        }

        madd!(
            map,
            "selected_area_fish",
            r.read_value_map_key(
                |r| r.read_namespaced_object().as_name(),
                |r, _| r.read_namespaced_object().into()
            )
        );

        madd!(
            map,
            "hiden_areas",
            r.read_set(|r| r.read_namespaced_object())
        );

        // Fishing contest
        if r.read_bool() {
            madd!(map, "fishing_contest", {
                let mut map = Map::new();

                madd!(
                    map,
                    "completion_tracker",
                    r.read_vector(|r| r.read_bool())
                );
                madd!(
                    map,
                    "mastery_tracker",
                    r.read_vector(|r| r.read_bool())
                );

                Into::<Value>::into(map)
            });
        }

        map.into()
    }

    fn decode_firemaking(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(map, "gathering_skill", r.decode_gathering_skill());

        madd!(map, "bonfire_timer", r.decode_timer());
        if r.read_bool() {
            madd!(map, "recipe", r.read_namespaced_object());
        }
        if r.read_bool() {
            madd!(map, "bonfire_recipe", r.read_namespaced_object());
        }
        madd!(map, "oil_timer", r.decode_timer());
        if r.read_bool() {
            madd!(map, "oil_log_recipe", r.read_namespaced_object());
        }
        if r.read_bool() {
            madd!(map, "selected_oil", r.read_namespaced_object());
        }

        map.into()
    }
}
