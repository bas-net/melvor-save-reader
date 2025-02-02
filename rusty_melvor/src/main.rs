use std::collections::HashMap;
// use std::fmt::Binary;
use std::fs::File;
// use core::str;
// use std::fs::File;
use std::io::{
    // self,
    self,
    Read, //  Write
};
use std::str;

// use base64::{engine::general_purpose::STANDARD, Engine as _};
// use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
// use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use base64::{prelude::BASE64_STANDARD, Engine as _};
use serde_json::{to_writer_pretty, Map, Number, Value};

trait IntoNumber {
    fn into_number(self) -> Number;
}

impl IntoNumber for u16 {
    fn into_number(self) -> Number {
        Number::from(self)
    }
}

impl IntoNumber for u8 {
    fn into_number(self) -> Number {
        Number::from(self)
    }
}

impl IntoNumber for f64 {
    fn into_number(self) -> Number {
        Number::from_f64(self).unwrap()
    }
}

trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for u16 {
    fn into_value(self) -> Value {
        Value::Number(Number::from(self))
    }
}

impl IntoValue for u32 {
    fn into_value(self) -> Value {
        Value::Number(Number::from(self))
    }
}

impl<T> IntoValue for Vec<T>
where
    T: IntoValue,
{
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(|item| item.into_value()).collect())
    }
}

#[derive(Eq, PartialEq, Hash)]
struct NamespacedObject {
    id: u16,
    text_id: Option<String>,
}

impl From<NamespacedObject> for Value {
    fn from(object: NamespacedObject) -> Value {
        let mut map = Map::new();
        map.insert("id".to_string(), object.id.into_value());
        match object.text_id {
            Some(text_id) => {
                map.insert("text_id".to_string(), Value::String(text_id))
            }
            None => None,
        };
        Value::Object(map)
    }
}

struct BankItem {
    item: NamespacedObject,
    quantity: u32,
}

impl IntoValue for BankItem {
    fn into_value(self) -> Value {
        let mut map = Map::new();
        map.insert("item".into(), self.item.into());
        map.insert("quantity".into(), self.quantity.into());
        Value::Object(map)
    }
}

fn main() {
    let skill = match std::env::args().nth(1) {
        Some(skill) => skill,
        None => {
            println!("Please provide a skill to train");
            return;
        }
    };

    open_save().unwrap_or(());

    println!("Hello, world!");
    println!("Training skill: {}", skill);
}

struct BinaryReader {
    data: Vec<u8>,
    byte_offset: usize,
    numeric_to_string_id_map: HashMap<u16, String>,
}

impl BinaryReader {
    fn read_buffer_of_size(&mut self, size: usize) -> Vec<u8> {
        let buffer =
            self.data[self.byte_offset..self.byte_offset + size].to_vec();
        self.byte_offset += size;

        // for byte in &buffer {
        //     match str::from_utf8(&[*byte]) {
        //         Ok(byte_str) => print!("{:02x}-{} ", byte, byte_str),
        //         Err(e) => print!("{:02x} ", byte),
        //     }
        // }
        // println!();

        buffer
    }

    fn read_uint32(&mut self) -> u32 {
        let buffer = self.read_buffer_of_size(4);
        let uint32 =
            u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

        uint32
    }

    fn read_uint16(&mut self) -> u16 {
        let buffer = self.read_buffer_of_size(2);
        let uint16 = u16::from_be_bytes([buffer[0], buffer[1]]);

        uint16
    }

    fn read_uint8(&mut self) -> u8 {
        let buffer = self.read_buffer_of_size(1);
        let uint8 = buffer[0];

        uint8
    }

    fn read_string(&mut self) -> String {
        let string_length = self.read_uint32();
        // println!("String length: {}", string_length);

        let string = match str::from_utf8(
            &self.data
                [self.byte_offset..self.byte_offset + string_length as usize],
        ) {
            Ok(string) => string,
            Err(e) => {
                println!("Failed to decode string: {}", e);
                return String::new();
            }
        };
        self.byte_offset += string_length as usize;
        string.to_string()
    }

    fn read_buffer(&mut self) -> Vec<u8> {
        let buffer_length = self.read_uint32();
        // println!("Buffer length: {}", buffer_length);

        let buffer = self.read_buffer_of_size(buffer_length as usize);

        buffer
    }

    fn read_float64(&mut self) -> f64 {
        let buffer = self.read_buffer_of_size(8);
        let float64 = f64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5],
            buffer[6], buffer[7],
        ]);

        float64
    }

    fn read_bool(&mut self) -> bool {
        let buffer = self.read_buffer_of_size(1);
        let boolean = buffer[0] == 1;

        boolean
    }

    fn read_set<T, F>(&mut self, read_value: F) -> Vec<T>
    where
        F: Fn(&mut Self) -> T,
    {
        let set_size = self.read_uint32();

        let mut set = Vec::new();
        for _ in 0..set_size {
            let value = read_value(self);
            set.push(value);
        }

        set
    }

    fn read_vector<T, F>(&mut self, read_value: F) -> Vec<T>
    where
        F: Fn(&mut Self) -> T,
    {
        let vector_size = self.read_uint32();

        let mut vector = Vec::new();
        for _ in 0..vector_size {
            let value = read_value(self);
            vector.push(value);
        }

        vector
    }

    fn read_map<K, V, F, G>(
        &mut self,
        read_key: F,
        read_value: G,
    ) -> HashMap<K, V>
    where
        F: Fn(&mut Self) -> K,
        G: Fn(&mut Self) -> V,
        K: Eq,
        K: std::hash::Hash,
    {
        let map_size = self.read_uint32();

        let mut map = HashMap::new();
        for _ in 0..map_size {
            let key = read_key(self);
            let value = read_value(self);
            map.insert(key, value);
        }

        map
    }

    fn read_value_map_key<F, G>(
        &mut self,
        read_key: F,
        read_value: G,
    ) -> Map<String, Value>
    where
        F: Fn(&mut Self) -> String,
        G: Fn(&mut Self, &String) -> Value,
    {
        let map_size = self.read_uint32();

        let mut map = Map::new();
        for _ in 0..map_size {
            let key = read_key(self);
            let value = read_value(self, &key);
            map.insert(key.into(), value.into());
        }

        map
    }

    fn validate_file_is_melvor_save(&mut self) -> bool {
        let buffer = self.read_buffer_of_size(6);
        let melvor = match str::from_utf8(buffer.as_slice()) {
            Ok(melvor) => melvor,
            Err(e) => {
                println!("Failed to decode save: {}", e);
                return false;
            }
        };

        return melvor == "melvor";
    }

    fn get_save_map_namedspaced_object(&mut self) -> NamespacedObject {
        let id = self.read_uint16();
        let text_id = match self.numeric_to_string_id_map.get(&id) {
            Some(text_id) => Some(text_id.to_string()),
            None => None,
        };

        let object = NamespacedObject { id, text_id };
        object
    }

    fn skip(&mut self, size: usize) {
        self.byte_offset += size;
    }
}

struct MelvorSaveReader {
    header: BinaryReader,
    raw_data: BinaryReader,
    save_map: HashMap<String, Value>,
}

impl MelvorSaveReader {
    fn read_save(data: Vec<u8>) -> Option<MelvorSaveReader> {
        let mut combined_reader = BinaryReader {
            data: data,
            byte_offset: 0,
            numeric_to_string_id_map: HashMap::new(),
        };

        if !combined_reader.validate_file_is_melvor_save() {
            println!("Failed to read save file header");
            return None;
        }

        let header = BinaryReader {
            data: combined_reader.read_buffer(),
            byte_offset: 0,
            numeric_to_string_id_map: HashMap::new(),
        };

        let raw_data = BinaryReader {
            data: combined_reader.read_buffer(),
            byte_offset: 0,
            numeric_to_string_id_map: HashMap::new(),
        };

        let mut save_reader = MelvorSaveReader {
            header: header,
            raw_data: raw_data,
            save_map: HashMap::new(),
        };

        let header_map = save_reader.read_header_map();
        // for (key, value) in header_map.iter() {
        //     println!("Key: {}", key);
        //     for (sub_key, sub_value) in value.iter() {
        //         println!("Sub Key: {}, Sub Value: {}", sub_key, sub_value);
        //     }
        // }
        write_hashmap_to_json(&header_map, "header_map.json").unwrap();

        let mut numeric_to_string_id_map = HashMap::new();
        for (namespace, value) in header_map.iter() {
            for (id, sub_value) in value.iter() {
                numeric_to_string_id_map
                    .insert(*sub_value, format!("{}:{}", namespace, id));
            }
        }
        write_hashmap_to_json(
            &numeric_to_string_id_map,
            "numeric_to_string_id_map.json",
        )
        .unwrap();

        save_reader.raw_data.numeric_to_string_id_map =
            numeric_to_string_id_map;

        let header_version = save_reader.header.read_uint32();
        println!("Header version: {}", header_version);

        save_reader.add_to_save_map_f64("tick_timestamp");
        save_reader.add_to_save_map_f64("save_timestamp");

        if save_reader.raw_data.read_bool() {
            save_reader.add_to_save_map_uint16("active_action_id");
        }

        if save_reader.raw_data.read_bool() {
            save_reader.add_to_save_map_uint16("paused_action_id");
        }

        save_reader.add_to_save_map_bool("is_paused");
        save_reader.add_to_save_map_bool("merchants_permit_read");
        save_reader.add_to_save_map_uint16("game_mode_id");
        save_reader.add_to_save_map_string("character_name");

        // Bank
        // Items that are in your bank and are locked so you can't sell them etc.
        save_reader
            .add_to_save_map_set("bank.locked_items", |r| r.read_uint16());

        save_reader.add_to_save_map_vector("bank.items_by_bank_tab", |r| {
            r.read_vector(|r| {
                let item = r.get_save_map_namedspaced_object();
                BankItem {
                    item: item,
                    quantity: r.read_uint32(),
                }
            })
        });

        save_reader.add_to_save_map("default_item_tabs", |r| -> Value {
            r.read_value_map_key(
                |r| {
                    let item = r.get_save_map_namedspaced_object();
                    match item.text_id {
                        Some(text_id) => text_id,
                        None => item.id.to_string(),
                    }
                },
                |r, _| r.read_uint8().into(),
            )
            .into()
        });

        // let custom_sort_order = save_reader
        //     .raw_data
        //     .read_vector(|r| r.get_save_map_namedspaced_object());

        save_reader.add_to_save_map("custom_sort_order", |r| {
            r.read_vector(|r| {
                let item = r.get_save_map_namedspaced_object();
                item
            })
        });
        // for (index, item_id) in custom_sort_order.iter().enumerate() {
        //     println!(
        //         "Custom sort order: {}, {}, {}",
        //         index,
        //         item_id.id,
        //         item_id.text_id.as_ref().unwrap_or(&"".to_string())
        //     );
        // }

        // let glowing_items = save_reader
        //     .raw_data
        //     .read_set(|r| r.get_save_map_namedspaced_object());

        save_reader.add_to_save_map("glowing_items", |r| {
            r.read_set(|r| r.get_save_map_namedspaced_object())
        });

        // for item_id in glowing_items.iter() {
        //     println!("Glowing item: {}", item_id.id);
        // }

        // let tab_icons = save_reader.raw_data.read_map(
        //     |r| r.read_uint8(),
        //     |r| r.get_save_map_namedspaced_object(),
        // );

        save_reader.add_to_save_map("tab_icons", |r| -> Value {
            r.read_value_map_key(
                |r| r.read_uint8().to_string(),
                |r, _| r.get_save_map_namedspaced_object().into(),
            )
            .into()
        });
        // for (tab_index, item) in tab_icons.iter() {
        //     println!("Tab icon: {}, {}", tab_index, item.id);
        // }

        // Combat Manager
        //   Base Manager
        //     Player
        //       Character
        // save_reader
        //     .add_to_save_map("combat_manager.hitpoints", |r| r.read_uint32());
        // save_reader.add_to_save_map("nextAction", |r| {
        //     if r.read_uint8() == 1 {
        //         "Attack"
        //     } else {
        //         "Nothing"
        //     }
        // });
        // save_reader.add_to_save_map("attack_count", |r| r.read_uint32());
        // save_reader.add_to_save_map_namedspaced_object("next_attack");
        // save_reader.add_to_save_map_bool("is_attacking");
        // save_reader.add_to_save_map_bool("first_hit");

        // // timers.act
        // save_reader.add_to_save_map("combat.timers.act", |r| read_timer(r));
        // save_reader.add_to_save_map("combat.timers.regen", |r| read_timer(r));

        // save_reader.add_to_save_map("combat.player.character", |r| {
        //     let mut map = Map::new();
        //     map.insert("turns_taken".into(), r.read_uint32().into());
        //     map.insert("buffered_regen".into(), r.read_uint32().into());
        //     // decode active effects
        //     map.insert(
        //         "active_effects".into(),
        //         r.read_value_map_key(
        //             |r| match r.get_save_map_namedspaced_object().text_id {
        //                 Some(text_id) => text_id,
        //                 None => "".to_string(),
        //             },
        //             |r, k| {
        //                 if k == "" {
        //                     // Skip
        //                     r.skip(18);
        //                     r.read_vector(|r| {
        //                         r.read_string();
        //                         r.skip(4);
        //                         Value::Null
        //                     });
        //                     r.read_vector(|r| {
        //                         r.read_string();
        //                         r.skip(4);
        //                         Value::Null
        //                     });
        //                     r.read_vector(|r| {
        //                         r.read_string();
        //                         r.skip(4);
        //                         Value::Null
        //                     });
        //                     r.read_vector(|r| {
        //                         r.read_string();
        //                         // Skip timer
        //                         r.skip(9);
        //                         Value::Null
        //                     });

        //                     Value::Null
        //                 } else {
        //                     // ActiveEffect decode
        //                     let mut map = Map::new();
        //                     map.insert(
        //                         "source_character".into(),
        //                         r.read_bool().into(),
        //                     );

        //                     let source = r.read_uint8();
        //                     map.insert(
        //                         "source".into(),
        //                         match source {
        //                             0 => "Attack".into(),
        //                             1 => "Effect".into(),
        //                             _ => "Other".into(),
        //                         },
        //                     );

        //                     map.insert(
        //                         "damageDealt".into(),
        //                         r.read_float64().into(),
        //                     );
        //                     map.insert(
        //                         "damageTaken".into(),
        //                         r.read_float64().into(),
        //                     );

        //                     map.insert(
        //                         "parameters".into(),
        //                         r.read_vector(|r| -> Value {
        //                             let mut param_map = Map::new();
        //                             param_map.insert(
        //                                 "name".into(),
        //                                 r.read_string().into(),
        //                             );
        //                             param_map.insert(
        //                                 "value".into(),
        //                                 r.read_uint32().into(),
        //                             );
        //                             param_map.into()
        //                         })
        //                         .into(),
        //                     );

        //                     map.insert(
        //                         "stat_groups".into(),
        //                         r.read_vector(|r| -> Value {
        //                             let mut map = Map::new();
        //                             map.insert(
        //                                 "name".into(),
        //                                 r.read_string().into(),
        //                             );
        //                             map.insert(
        //                                 "value".into(),
        //                                 r.read_uint32().into(),
        //                             );
        //                             map.into()
        //                         })
        //                         .into(),
        //                     );

        //                     map.insert(
        //                         "timers".into(),
        //                         r.read_vector(|r| -> Value {
        //                             let mut map = Map::new();
        //                             map.insert(
        //                                 "name".into(),
        //                                 r.read_string().into(),
        //                             );
        //                             map.insert("timer".into(), read_timer(r));
        //                             map.into()
        //                         })
        //                         .into(),
        //                     );

        //                     map.into()
        //                 }
        //             },
        //         )
        //         .into(),
        //     );

        //     map.insert("first_miss".into(), r.read_bool().into());
        //     map.insert("barrier".into(), r.read_uint32().into());

        //     map
        // });

        save_reader.add_to_save_map("combat.player.character", |r| {
            decode_character(r)
        });

        save_reader.add_to_save_map("combat.player", |r| {
            let mut map = Map::new();
            // Melee
            if r.read_bool() {
                map.insert(
                    "melee_style".into(),
                    r.get_save_map_namedspaced_object().into(),
                );
            }
            // Ranged
            if r.read_bool() {
                map.insert(
                    "ranged_style".into(),
                    r.get_save_map_namedspaced_object().into(),
                );
            }
            // Magic
            if r.read_bool() {
                map.insert(
                    "magic_style".into(),
                    r.get_save_map_namedspaced_object().into(),
                );
            }
            map.insert("prayer_points".into(), r.read_uint32().into());
            map.insert(
                "selected_equipment_set".into(),
                r.read_uint16().into(),
            );

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
                                r.get_save_map_namedspaced_object().into(),
                            );
                            if r.read_bool() {
                                map.insert(
                                    "item".into(),
                                    r.get_save_map_namedspaced_object().into(),
                                );
                                map.insert(
                                    "quantity".into(),
                                    r.read_uint32().into(),
                                );
                            }
                            map.insert(
                                "quick_equip_items".into(),
                                r.read_vector(|r| -> Value {
                                    r.get_save_map_namedspaced_object().into()
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
                            r.get_save_map_namedspaced_object().into(),
                        );
                    }
                    if r.read_bool() {
                        map.insert(
                            "aurora".into(),
                            r.get_save_map_namedspaced_object().into(),
                        );
                    }
                    if r.read_bool() {
                        map.insert(
                            "curse".into(),
                            r.get_save_map_namedspaced_object().into(),
                        );
                    }

                    // Prayer selection (set)
                    map.insert(
                        "prayers".into(),
                        r.read_set(|r| -> Value {
                            r.get_save_map_namedspaced_object().into()
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
                            r.get_save_map_namedspaced_object().into(),
                        );
                        map.insert("quantity".into(), r.read_uint32().into());
                        map.into()
                    })
                    .into(),
                );

                map.into()
            });

            // player.timers.summon
            map.insert("summon_timer".into(), read_timer(r));

            map.insert("soul_points".into(), r.read_uint32().into());
            map.insert(
                "unholy_prayer_multiplier".into(),
                r.read_uint8().into(),
            );

            map
        });

        // combat_manager.base_manager.enemy.character
        save_reader.add_to_save_map(
            "combat_manager.base_manager.enemy.character",
            |r| decode_character(r),
        );

        // combat_manager.base_manager.enemy
        save_reader.add_to_save_map(
            "combat_manager.base_manager.enemy",
            |r| {
                let mut map = Map::new();
                map.insert("state".into(), r.read_uint8().into());
                map.insert("random_attack_type".into(), {
                    match r.read_uint8() {
                        0 => "Melee",
                        1 => "Ranged",
                        2 => "Magic",
                        3 => "Unset",
                        _ => "Other",
                    }
                    .into()
                });

                if r.read_bool() {
                    map.insert(
                        "monster".into(),
                        r.get_save_map_namedspaced_object().into(),
                    );
                }

                if r.read_bool() {
                    map.insert(
                        "override_damage_type".into(),
                        r.get_save_map_namedspaced_object().into(),
                    );
                }

                map
            },
        );

        // combat_manager.base_manager
        save_reader.add_to_save_map("combat_manager.base_manager", |r| {
            let mut map = Map::new();
            map.insert("fight_in_progress".into(), r.read_bool().into());
            map.insert("spawn_timer".into(), read_timer(r));
            map.insert("is_active".into(), r.read_bool().into());

            // decode passives
            map.insert(
                "passives".into(),
                r.read_vector(|r| -> Value {
                    let passive = r.get_save_map_namedspaced_object();
                    let mut passive_map = Map::new();
                    passive_map.insert("passive".into(), passive.into());
                    passive_map.insert("display".into(), r.read_bool().into());
                    passive_map.into()
                })
                .into(),
            );

            map
        });

        // combat_manager
        save_reader.add_to_save_map("combat_manager", |r| {
            let mut map = Map::new();

            if r.read_bool() {
                let selected_area_type = r.read_uint8();
                map.insert(
                    "selected_area_type".into(),
                    selected_area_type.into(),
                );
                map.insert(
                    "selected_area".into(),
                    r.get_save_map_namedspaced_object().into(),
                );
            }

            map.insert("area_progress".into(), r.read_uint32().into());

            if r.read_bool() {
                map.insert(
                    "selected_monster".into(),
                    r.get_save_map_namedspaced_object().into(),
                );
            }

            map.insert("paused".into(), r.read_bool().into());

            // combat_manager.loot
            map.insert("loot".into(), {
                r.read_vector(|r| -> Value {
                    let mut map = Map::new();
                    map.insert(
                        "item".into(),
                        r.get_save_map_namedspaced_object().into(),
                    );
                    map.insert("quantity".into(), r.read_uint32().into());
                    map.into()
                })
                .into()
            });

            // combat_manager.slayer_task
            map.insert("slayer_task".into(), {
                let mut map = Map::new();
                map.insert("active".into(), r.read_bool().into());

                if r.read_bool() {
                    map.insert(
                        "monster".into(),
                        r.get_save_map_namedspaced_object().into(),
                    );
                }

                map.insert("kills_left".into(), r.read_uint32().into());
                map.insert("extended".into(), r.read_bool().into());

                if r.read_bool() {
                    map.insert(
                        "category".into(),
                        r.get_save_map_namedspaced_object().into(),
                    );
                }

                map.insert(
                    "categories".into(),
                    r.read_vector(|r| -> Value {
                        let mut map = Map::new();
                        map.insert(
                            "category".into(),
                            r.get_save_map_namedspaced_object().into(),
                        );
                        map.insert(
                            "tasks_completed".into(),
                            r.read_uint32().into(),
                        );

                        map.into()
                    })
                    .into(),
                );

                map.insert("task_timer".into(), read_timer(r));

                map.insert("realm".into(), r.get_save_map_namedspaced_object().into());

                map.into()
            });

            map
        });

        write_hashmap_to_json(&save_reader.save_map, "save_map.json").unwrap();

        return Some(save_reader);
    }

    fn read_header_map(&mut self) -> HashMap<String, HashMap<String, u16>> {
        let map_size = self.header.read_uint32();

        let mut map = HashMap::new();

        // println!("Map size: {}", map_size);

        for _ in 0..map_size {
            let key = self.header.read_string();

            // println!("Key: {}", key);

            let sub_map_size = self.header.read_uint32();
            let mut sub_map = HashMap::new();
            for _ in 0..sub_map_size {
                let sub_key = self.header.read_string();
                // println!("  Sub Key: {}", sub_key);
                let sub_value = self.header.read_uint16();
                // println!("  Sub Value: {}", sub_value);
                sub_map.insert(sub_key, sub_value);
            }
            map.insert(key, sub_map);
        }

        map
    }

    fn add_to_save_map_f64(&mut self, key: &str) {
        self.save_map.insert(
            key.to_string(),
            Value::Number(
                Number::from_f64(self.raw_data.read_float64()).unwrap(),
            ),
        );
    }

    fn add_to_save_map_uint16(&mut self, key: &str) {
        self.save_map.insert(
            key.to_string(),
            Value::Number(Number::from(self.raw_data.read_uint16())),
        );
    }

    fn add_to_save_map_bool(&mut self, key: &str) {
        self.save_map
            .insert(key.to_string(), Value::Bool(self.raw_data.read_bool()));
    }

    fn add_to_save_map_string(&mut self, key: &str) {
        self.save_map.insert(
            key.to_string(),
            Value::String(self.raw_data.read_string()),
        );
    }

    fn add_to_save_map_set<T, F>(&mut self, key: &str, read_value: F)
    where
        T: IntoNumber,
        F: Fn(&mut BinaryReader) -> T,
    {
        self.save_map.insert(
            key.to_string(),
            Value::Array(
                self.raw_data
                    .read_set(|r| Value::Number(read_value(r).into_number())),
            ),
        );
    }

    fn add_to_save_map_vector<T, F>(&mut self, key: &str, read_value: F)
    where
        T: IntoValue,
        F: Fn(&mut BinaryReader) -> T,
    {
        self.save_map.insert(
            key.to_string(),
            Value::Array(
                self.raw_data.read_vector(|r| read_value(r).into_value()),
            ),
        );
    }

    fn add_to_save_map<T, F>(&mut self, key: &str, read_value: F)
    where
        T: Into<Value>,
        F: Fn(&mut BinaryReader) -> T,
    {
        self.save_map
            .insert(key.to_string(), read_value(&mut self.raw_data).into());
    }

    fn add_to_save_map_namedspaced_object(&mut self, key: &str) {
        let object = self.raw_data.get_save_map_namedspaced_object();

        self.save_map.insert(key.to_string(), object.into());
    }
}

fn open_save() -> Option<()> {
    // Open file save.txt
    let save = match std::fs::read_to_string("save2.txt") {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read save.txt: {}", e);
            return None;
        }
    };

    let zlib_bytes = match BASE64_STANDARD.decode(save.trim()) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("Failed to decode base64: {}", e);
            return None;
        }
    };
    let mut decoder = flate2::read::ZlibDecoder::new(&zlib_bytes[..]);
    let mut save = Vec::new();
    match decoder.read_to_end(&mut save) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to decode save: {}", e);
            // Print the first 100 bytes of the gzip
            for byte in &zlib_bytes[..25] {
                print!("{:02x} ", byte);
            }
            return None;
        }
    }

    let _save_reader = MelvorSaveReader::read_save(save.clone());

    Some(())
}

fn write_hashmap_to_json<K: serde::Serialize, V: serde::Serialize>(
    map: &HashMap<K, V>,
    filename: &str,
) -> io::Result<()> {
    let file = File::create(filename)?;
    to_writer_pretty(file, map)?;
    Ok(())
}

fn read_timer(reader: &mut BinaryReader) -> Value {
    let mut map = Map::new();
    map.insert("ticksleft".into(), reader.read_uint32().into());
    map.insert("maxticks".into(), reader.read_uint32().into());
    map.insert("active".into(), reader.read_bool().into());
    map.into()
}

fn decode_character(r: &mut BinaryReader) -> Value {
    let mut map = Map::new();
    map.insert("combat_manager.hitpoints".into(), r.read_uint32().into());
    map.insert(
        "nextAction".into(),
        {
            match r.read_uint8() {
                1 => "Attack",
                _ => "Nothing",
            }
        }
        .into(),
    );

    map.insert("attack_count".into(), r.read_uint32().into());
    map.insert(
        "next_attack".into(),
        r.get_save_map_namedspaced_object().into(),
    );
    map.insert("is_attacking".into(), r.read_bool().into());
    map.insert("first_hit".into(), r.read_bool().into());

    map.insert("timers_act".into(), read_timer(r));
    map.insert("timers_regen".into(), read_timer(r));

    map.insert("turns_taken".into(), r.read_uint32().into());
    map.insert("buffered_regen".into(), r.read_uint32().into());
    // decode active effects
    map.insert(
        "active_effects".into(),
        r.read_value_map_key(
            |r| match r.get_save_map_namedspaced_object().text_id {
                Some(text_id) => text_id,
                None => "".to_string(),
            },
            |r, k| {
                if k == "" {
                    // Skip
                    r.skip(18);
                    r.read_vector(|r| {
                        r.read_string();
                        r.skip(4);
                        Value::Null
                    });
                    r.read_vector(|r| {
                        r.read_string();
                        r.skip(4);
                        Value::Null
                    });
                    r.read_vector(|r| {
                        r.read_string();
                        r.skip(4);
                        Value::Null
                    });
                    r.read_vector(|r| {
                        r.read_string();
                        // Skip timer
                        r.skip(9);
                        Value::Null
                    });

                    Value::Null
                } else {
                    // ActiveEffect decode
                    let mut map = Map::new();
                    map.insert(
                        "source_character".into(),
                        r.read_bool().into(),
                    );

                    let source = r.read_uint8();
                    map.insert(
                        "source".into(),
                        match source {
                            0 => "Attack".into(),
                            1 => "Effect".into(),
                            _ => "Other".into(),
                        },
                    );

                    map.insert("damageDealt".into(), r.read_float64().into());
                    map.insert("damageTaken".into(), r.read_float64().into());

                    map.insert(
                        "parameters".into(),
                        r.read_vector(|r| -> Value {
                            let mut param_map = Map::new();
                            param_map
                                .insert("name".into(), r.read_string().into());
                            param_map.insert(
                                "value".into(),
                                r.read_uint32().into(),
                            );
                            param_map.into()
                        })
                        .into(),
                    );

                    map.insert(
                        "stat_groups".into(),
                        r.read_vector(|r| -> Value {
                            let mut map = Map::new();
                            map.insert("name".into(), r.read_string().into());
                            map.insert("value".into(), r.read_uint32().into());
                            map.into()
                        })
                        .into(),
                    );

                    map.insert(
                        "timers".into(),
                        r.read_vector(|r| -> Value {
                            let mut map = Map::new();
                            map.insert("name".into(), r.read_string().into());
                            map.insert("timer".into(), read_timer(r));
                            map.into()
                        })
                        .into(),
                    );

                    map.into()
                }
            },
        )
        .into(),
    );

    map.insert("first_miss".into(), r.read_bool().into());
    map.insert("barrier".into(), r.read_uint32().into());

    map.into()
}
