use std::collections::HashMap;
use std::fmt::Binary;
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
use serde_json::to_writer_pretty;

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
}

struct MelvorSaveReader {
    header: BinaryReader,
    raw_data: BinaryReader,
}

impl MelvorSaveReader {
    fn read_save(data: Vec<u8>) -> Option<MelvorSaveReader> {
        let mut combined_reader = BinaryReader {
            data: data,
            byte_offset: 0,
        };

        if !combined_reader.validate_file_is_melvor_save() {
            println!("Failed to read save file header");
            return None;
        }

        let header = BinaryReader {
            data: combined_reader.read_buffer(),
            byte_offset: 0,
        };

        let raw_data = BinaryReader {
            data: combined_reader.read_buffer(),
            byte_offset: 0,
        };

        let mut save_reader = MelvorSaveReader {
            header: header,
            raw_data: raw_data,
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

        let header_version = save_reader.header.read_uint32();
        println!("Header version: {}", header_version);

        let tick_timestamp = save_reader.raw_data.read_float64();
        println!("Tick timestamp: {}", tick_timestamp);

        let save_timestamp = save_reader.raw_data.read_float64();
        println!("Save timestamp: {}", save_timestamp);

        if save_reader.raw_data.read_bool() {
            let active_action_id = save_reader.raw_data.read_uint16();
            println!(
                "Active action id: {}, {}",
                active_action_id,
                numeric_to_string_id_map.get(&active_action_id).unwrap()
            );
        }

        if save_reader.raw_data.read_bool() {
            let paused_action_id = save_reader.raw_data.read_uint16();
            println!(
                "Paused action id: {}, {}",
                paused_action_id,
                numeric_to_string_id_map.get(&paused_action_id).unwrap()
            );
        }

        let is_paused = save_reader.raw_data.read_bool();
        println!("Is paused: {}", is_paused);

        let merchants_permit_read = save_reader.raw_data.read_bool();
        println!("Merchants permit read: {}", merchants_permit_read);

        let game_mode = save_reader.raw_data.read_uint16();
        println!(
            "Game mode: {}, {}",
            game_mode,
            numeric_to_string_id_map.get(&game_mode).unwrap()
        );

        let character_name = save_reader.raw_data.read_string();
        println!("Character name: {}", character_name);

        // Bank
        // Items that are in your bank and are locked so you can't sell them etc.
        let locked_items = save_reader.raw_data.read_set(|r| r.read_uint16());
        for item_id in locked_items.iter() {
            println!(
                "Locked item id: {}, {}",
                item_id,
                numeric_to_string_id_map.get(item_id).unwrap()
            );
        }

        let items_by_tab = save_reader.raw_data.read_vector(|reader| {
            reader.read_vector(|reader| {
                let item = reader.read_uint16();
                let quantity = reader.read_uint32();
                (item, quantity)
            })
        });
        for (tab_index, tab) in items_by_tab.iter().enumerate() {
            for (item_id, quantity) in tab.iter() {
                println!(
                    "Tab: {}, Item: {}, {}, Quantity: {}",
                    tab_index,
                    item_id,
                    numeric_to_string_id_map.get(item_id).unwrap(),
                    quantity
                );
            }
        }

        let default_item_tabs = save_reader
            .raw_data
            .read_map(|r| r.read_uint16(), |r| r.read_uint8());
        for (tab_index, tab) in default_item_tabs.iter() {
            println!(
                "Default item tab: {}, {}, {}",
                tab_index,
                numeric_to_string_id_map.get(tab_index).unwrap(),
                tab
            );
        }

        let custom_sort_order =
            save_reader.raw_data.read_vector(|r| r.read_uint16());
        for (index, item_id) in custom_sort_order.iter().enumerate() {
            println!(
                "Custom sort order: {}, {}, {}",
                index,
                item_id,
                numeric_to_string_id_map.get(item_id).unwrap()
            );
        }

        let glowing_items = save_reader.raw_data.read_set(|r| r.read_uint16());
        for item_id in glowing_items.iter() {
            println!(
                "Glowing item: {}, {}",
                item_id,
                numeric_to_string_id_map.get(item_id).unwrap()
            );
        }

        let tab_icons = save_reader
            .raw_data
            .read_map(|r| r.read_uint8(), |r| r.read_uint16());
        for (tab_index, icon_id) in tab_icons.iter() {
            println!(
                "Tab icon: {}, {}, {}",
                tab_index,
                icon_id,
                numeric_to_string_id_map.get(icon_id).unwrap()
            );
        }



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
}

fn open_save() -> Option<()> {
    // Open file save.txt
    let save = match std::fs::read_to_string("save.txt") {
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

    let save_reader = MelvorSaveReader::read_save(save.clone());

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
