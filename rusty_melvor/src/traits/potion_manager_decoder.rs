use serde_json::{Map, Value};

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait PotionManagerDecoder: DataReaders {
    fn decode_potion_manager(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(
            map,
            "active_potions",
            r.read_value_map_key(
                |r| {
                    let item = r.read_namespaced_object();
                    match item.text_id {
                        Some(text_id) => text_id,
                        None => item.id.to_string(),
                    }
                },
                |r, _| {
                    let mut map = Map::new();
                    madd!(map, "potion", r.read_namespaced_object());
                    madd!(map, "charges", r.read_uint32());
                    map.into()
                }
            )
        );

        madd!(
            map,
            "auto_reuse_actions",
            r.read_set(|r| -> Value { r.read_namespaced_object().into() })
        );

        map.into()
    }
}
