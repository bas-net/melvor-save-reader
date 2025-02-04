use serde_json::{Map, Value};

use crate::NamespacedObject;

pub trait ByteOffset {
    fn get_byte_offset(&self) -> usize;
    fn increment_byte_offset(&mut self, increment: usize);

    fn print_previous_bytes(&self, size: usize);
    fn print_next_bytes(&self, size: usize);
}

pub trait Buffer {
    fn get_slice(&mut self, start: usize, end: usize) -> Vec<u8>;
}

pub trait HasNumericToStringIdMap {
    fn map_numeric_to_string_id(&self, id: &u16) -> Option<String>;
}

pub trait BufferReader: Buffer + ByteOffset {
    fn read_buffer_by_size(&mut self, size: usize) -> Vec<u8> {
        let start = self.get_byte_offset();
        let end = start + size;
        let buffer = self.get_slice(start, end);

        self.increment_byte_offset(size);

        buffer
    }

    fn skip(&mut self, size: usize) {
        self.increment_byte_offset(size);
    }
}

pub trait DataReaders: BufferReader + HasNumericToStringIdMap {
    fn read_uint8(&mut self) -> u8 {
        let buffer = self.read_buffer_by_size(1);
        buffer[0]
    }
    fn read_uint16(&mut self) -> u16 {
        let buffer = self.read_buffer_by_size(2);
        u16::from_be_bytes([buffer[0], buffer[1]])
    }
    fn read_uint32(&mut self) -> u32 {
        let buffer = self.read_buffer_by_size(4);

        // println!("u32: {}", u);
        u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]])
    }
    fn read_float64(&mut self) -> f64 {
        let buffer = self.read_buffer_by_size(8);
        f64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5],
            buffer[6], buffer[7],
        ])
    }
    fn read_string(&mut self) -> String {
        let size = self.read_uint32() as usize;
        let buffer = self.read_buffer_by_size(size);
        match String::from_utf8(buffer) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to decode string: {}", e);
                "".to_string()
            }
        }
    }
    fn read_buffer(&mut self) -> Vec<u8> {
        let size = self.read_uint32() as usize;

        self.read_buffer_by_size(size)
    }
    fn read_bool(&mut self) -> bool {
        self.read_uint8() == 1
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
            map.insert(key, value);
        }

        map
    }

    fn read_namespaced_object(&mut self) -> NamespacedObject {
        let id = self.read_uint16();
        let text_id = self
            .map_numeric_to_string_id(&id)
            .map(|text_id| text_id.to_string());

        NamespacedObject { id, text_id }
    }
}
