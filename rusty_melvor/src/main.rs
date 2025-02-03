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
use traits::bank_decoder::BankDecoder;
use traits::base_manager_decoder::BaseManagerDecoder;
use traits::character_decoder::CharacterDecoder;
use traits::enemy_decoder::EnemyDecoder;
use traits::player_decoder::PlayerDecoder;
use traits::raid_enemy_decoder::RaidEnemyDecoder;
use traits::raid_manager_decoder::RaidManagerDecoder;
use traits::raid_player_decoder::RaidPlayerDecoder;
use traits::read::{
    Buffer, BufferReader, ByteOffset, DataReaders, HasNumericToStringIdMap,
};
use traits::timer_decoder::TimerDecoder;

mod traits;

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

#[derive(PartialEq, Hash)]
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

trait AutoInto {
    fn auto_insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<Value>;
}

impl AutoInto for Map<String, Value> {
    fn auto_insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.insert(key.into(), value.into());
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

impl Buffer for BinaryReader {
    fn get_slice(&mut self, start: usize, end: usize) -> Vec<u8> {
        self.data[start..end].to_vec()
    }
}

impl ByteOffset for BinaryReader {
    fn get_byte_offset(&self) -> usize {
        self.byte_offset
    }

    fn increment_byte_offset(&mut self, increment: usize) {
        self.byte_offset += increment;
        // println!("Byte offset: {}", self.byte_offset);
    }
}

impl HasNumericToStringIdMap for BinaryReader {
    fn map_numeric_to_string_id(&self, id: &u16) -> Option<String> {
        match self.numeric_to_string_id_map.get(id) {
            Some(text_id) => Some(text_id.to_string()),
            None => None,
        }
    }
}

impl BufferReader for BinaryReader {}
impl DataReaders for BinaryReader {}

impl TimerDecoder for BinaryReader {}
impl CharacterDecoder for BinaryReader {}
impl PlayerDecoder for BinaryReader {}
impl RaidPlayerDecoder for BinaryReader {}
impl BaseManagerDecoder for BinaryReader {}
impl EnemyDecoder for BinaryReader {}
impl RaidEnemyDecoder for BinaryReader {}
impl RaidManagerDecoder for BinaryReader {}

impl BankDecoder for BinaryReader {}

impl BinaryReader {
    fn validate_file_is_melvor_save(&mut self) -> bool {
        let buffer = self.read_buffer_by_size(6);
        let melvor = match str::from_utf8(buffer.as_slice()) {
            Ok(melvor) => melvor,
            Err(e) => {
                println!("Failed to decode save: {}", e);
                return false;
            }
        };

        return melvor == "melvor";
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

        save_reader.add_to_save_map("bank", |r| r.decode_bank());

        save_reader.add_to_save_map("combat_manager.base_manager", |r| {
            r.decode_base_manager(|r| r.decode_player(), |r| r.decode_enemy())
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

                map.insert(
                    "realm".into(),
                    r.get_save_map_namedspaced_object().into(),
                );

                map.into()
            });

            if r.read_bool() {
                map.insert(
                    "active_event".into(),
                    r.get_save_map_namedspaced_object().into(),
                );
            }

            map.insert(
                "event_passives".into(),
                r.read_vector(|r| -> Value {
                    r.get_save_map_namedspaced_object().into()
                })
                .into(),
            );

            map.auto_insert(
                "event_passives_being_selected",
                r.read_set(|r| r.get_save_map_namedspaced_object()),
            );

            map.auto_insert("event_dungon_length", r.read_uint32());

            map.auto_insert(
                "active_event_areas",
                r.read_value_map_key(
                    |r| match r.get_save_map_namedspaced_object() {
                        NamespacedObject {
                            text_id: Some(text_id),
                            ..
                        } => text_id,
                        NamespacedObject { id, .. } => id.to_string(),
                    },
                    |r, _| r.read_uint32().into(),
                ),
            );

            map.auto_insert("event_progress", r.read_uint32());

            map.auto_insert(
                "dungeon_completion",
                r.read_value_map_key(
                    |r| match r.get_save_map_namedspaced_object() {
                        NamespacedObject {
                            text_id: Some(text_id),
                            ..
                        } => text_id,
                        NamespacedObject { id, .. } => id.to_string(),
                    },
                    |r, _| r.read_uint32().into(),
                ),
            );

            map.auto_insert("stronghold_tier", r.read_uint8());

            map
        });

        // raid_manager
        save_reader
            .add_to_save_map("raid_manager", |r| r.decode_raid_manager());

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

    // fn add_to_save_map_set<T, F>(&mut self, key: &str, read_value: F)
    // where
    //     T: IntoNumber,
    //     F: Fn(&mut BinaryReader) -> T,
    // {
    //     self.save_map.insert(
    //         key.to_string(),
    //         Value::Array(
    //             self.raw_data
    //                 .read_set(|r| Value::Number(read_value(r).into_number())),
    //         ),
    //     );
    // }

    // fn add_to_save_map_vector<T, F>(&mut self, key: &str, read_value: F)
    // where
    //     T: IntoValue,
    //     F: Fn(&mut BinaryReader) -> T,
    // {
    //     self.save_map.insert(
    //         key.to_string(),
    //         Value::Array(
    //             self.raw_data.read_vector(|r| read_value(r).into_value()),
    //         ),
    //     );
    // }

    fn add_to_save_map<T, F>(&mut self, key: &str, read_value: F)
    where
        T: Into<Value>,
        F: Fn(&mut BinaryReader) -> T,
    {
        self.save_map
            .insert(key.to_string(), read_value(&mut self.raw_data).into());
    }

    // fn add_to_save_map_namedspaced_object(&mut self, key: &str) {
    //     let object = self.raw_data.get_save_map_namedspaced_object();

    //     self.save_map.insert(key.to_string(), object.into());
    // }
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

// fn decode_character(r: &mut BinaryReader) -> Value {
//     let mut map = Map::new();
//     map.insert("combat_manager.hitpoints".into(), r.read_uint32().into());
//     map.insert(
//         "nextAction".into(),
//         {
//             match r.read_uint8() {
//                 1 => "Attack",
//                 _ => "Nothing",
//             }
//         }
//         .into(),
//     );

//     map.insert("attack_count".into(), r.read_uint32().into());
//     map.insert(
//         "next_attack".into(),
//         r.get_save_map_namedspaced_object().into(),
//     );
//     map.insert("is_attacking".into(), r.read_bool().into());
//     map.insert("first_hit".into(), r.read_bool().into());

//     map.insert("timers_act".into(), read_timer(r));
//     map.insert("timers_regen".into(), read_timer(r));

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

//                     map.insert("damageDealt".into(), r.read_float64().into());
//                     map.insert("damageTaken".into(), r.read_float64().into());

//                     map.insert(
//                         "parameters".into(),
//                         r.read_vector(|r| -> Value {
//                             let mut param_map = Map::new();
//                             param_map
//                                 .insert("name".into(), r.read_string().into());
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
//                             map.insert("name".into(), r.read_string().into());
//                             map.insert("value".into(), r.read_uint32().into());
//                             map.into()
//                         })
//                         .into(),
//                     );

//                     map.insert(
//                         "timers".into(),
//                         r.read_vector(|r| -> Value {
//                             let mut map = Map::new();
//                             map.insert("name".into(), r.read_string().into());
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

//     map.into()
// }
