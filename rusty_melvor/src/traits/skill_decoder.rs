use serde_json::{Map, Value};

use crate::NamespacedObject;

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait SkillDecoder: DataReaders {
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
}
