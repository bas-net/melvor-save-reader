use serde_json::{Map, Value};

use super::read::DataReaders;

pub trait ItemChargesDecoder: DataReaders {
    fn decode_item_charges(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert(
            "charges".into(),
            r.read_value_map_key(
                |r| {
                    let item = r.read_namespaced_object();
                    match item.text_id {
                        Some(text_id) => text_id,
                        None => item.id.to_string(),
                    }
                },
                |r, _| r.read_uint32().into(),
            )
            .into(),
        );

        map.into()
    }
}
