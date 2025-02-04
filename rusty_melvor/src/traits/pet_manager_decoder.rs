use serde_json::{Map, Value};

use super::read::DataReaders;

pub trait PetManagerDecoder: DataReaders {
    fn decode_pet_manager(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert(
            "unlocked".into(),
            r.read_set(|r| -> Value {
                r.read_namespaced_object().into()
            })
            .into(),
        );

        map.into()
    }
}
