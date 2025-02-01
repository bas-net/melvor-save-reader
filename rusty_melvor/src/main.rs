use std::collections::HashMap;
use std::fmt::Binary;
// use core::str;
// use std::fs::File;
use std::io::{
    // self,
    Read,
    //  Write
};
use std::str;

// use base64::{engine::general_purpose::STANDARD, Engine as _};
// use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
// use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use base64::{prelude::BASE64_STANDARD, Engine as _};

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

    fn read_uing16(&mut self) -> u16 {
        let buffer = self.read_buffer_of_size(2);
        let uint16 = u16::from_be_bytes([buffer[0], buffer[1]]);

        uint16
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
        for (key, value) in header_map.iter() {
            println!("Key: {}", key);
            for (sub_key, sub_value) in value.iter() {
                println!("Sub Key: {}, Sub Value: {}", sub_key, sub_value);
            }
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
                let sub_value = self.header.read_uing16();
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

    MelvorSaveReader::read_save(save.clone());

    // write_output_to_file(&save).unwrap_or(());

    // let mut byte_offset = 0;

    // let melvor = match str::from_utf8(&save[byte_offset..byte_offset + 6]) {
    //     Ok(melvor) => melvor,
    //     Err(e) => {
    //         println!("Failed to decode save: {}", e);
    //         return None;
    //     }
    // };
    // byte_offset += 6;

    // let header_length = u32::from_be_bytes([
    //     save[byte_offset],
    //     save[byte_offset + 1],
    //     save[byte_offset + 2],
    //     save[byte_offset + 3],
    // ]);
    // byte_offset += 4;

    // let header_data =
    //     save[byte_offset..byte_offset + header_length as usize].to_vec();
    // byte_offset += header_length as usize;

    // let mut header_offset = 0;
    // // Map size
    // let header_map_size = u32::from_be_bytes([
    //     header_data[header_offset],
    //     header_data[header_offset + 1],
    //     header_data[header_offset + 2],
    //     header_data[header_offset + 3],
    // ]);
    // header_offset += 4;

    // println!("Map size: {}", header_map_size);
    // // let mut header_map: HashMap<&str, _> = HashMap::<&str, _>::new();
    // for i in 0..header_map_size {
    //     // Map key
    //     // Get string length
    //     let map_key_length = u32::from_be_bytes([
    //         header_data[header_offset],
    //         header_data[header_offset + 1],
    //         header_data[header_offset + 2],
    //         header_data[header_offset + 3],
    //     ]);
    //     header_offset += 4;

    //     let map_key = match str::from_utf8(
    //         &header_data
    //             [header_offset..header_offset + map_key_length as usize],
    //     ) {
    //         Ok(map_key) => map_key,
    //         Err(e) => {
    //             println!("Failed to decode map key: {}", e);
    //             return None;
    //         }
    //     };
    //     header_offset += map_key_length as usize;

    //     // Map value
    //     // ValueKey
    //     let value_key_length = u32::from_be_bytes([
    //         header_data[header_offset],
    //         header_data[header_offset + 1],
    //         header_data[header_offset + 2],
    //         header_data[header_offset + 3],
    //     ]);
    //     header_offset += 4;

    //     let value_key_header = match str::from_utf8(
    //         &header_data
    //             [header_offset..header_offset + value_key_length as usize],
    //     ) {
    //         Ok(value_key_header) => value_key_header,
    //         Err(e) => {
    //             println!("Failed to decode value key header: {}", e);
    //             return None;
    //         }
    //     };

    //     println!("Map key: {}", map_key);
    //     break;
    // }

    // let save_data = save[byte_offset..].to_vec();

    // let uint32_header = u32::from_be_bytes([
    //     save[byte_offset],
    //     save[byte_offset + 1],
    //     save[byte_offset + 2],
    //     save[byte_offset + 3],
    // ]);
    // byte_offset += 4;

    // let uint32_savewriter = u32::from_be_bytes([
    //     save[byte_offset],
    //     save[byte_offset + 1],
    //     save[byte_offset + 2],
    //     save[byte_offset + 3],
    // ]);
    // byte_offset += 4;

    // println!("Melvor: {}", melvor);
    // println!("Header: {}", header_length);
    // println!("Savewriter: {}", uint32_savewriter);

    // println!("{}", save);

    Some(())
}

// fn write_output_to_file(data: &[u8]) -> io::Result<()> {
//     let mut file = File::create("output.txt")?;
//     let mut i = 0;
//     while i < data.len() {
//         match str::from_utf8(&data[i..]) {
//             Ok(valid_str) => {
//                 writeln!(file, "Valid UTF-8: {}", valid_str)?;
//                 break;
//             }
//             Err(e) => {
//                 let valid_up_to = e.valid_up_to();
//                 if valid_up_to > 0 {
//                     writeln!(
//                         file,
//                         "Valid UTF-8: {}",
//                         str::from_utf8(&data[i..i + valid_up_to])
//                             .unwrap()
//                             .to_string()
//                     )?;
//                     i += valid_up_to;
//                 }
//                 writeln!(file, "Invalid UTF-8: {:02x}", data[i])?;
//                 i += 1;
//             }
//         }
//     }
//     Ok(())
// }
