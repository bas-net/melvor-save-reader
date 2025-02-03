use serde_json::{Map, Value};

use crate::NamespacedObject;

use super::read::DataReaders;

pub trait MinibarDecoder: DataReaders {
    fn decode_minibar(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert(
            "custom_items".into(),
            r.read_value_map_key(
                |r| match r.get_save_map_namedspaced_object() {
                    NamespacedObject {
                        text_id: Some(text_id),
                        ..
                    } => text_id.into(),
                    NamespacedObject { id, .. } => id.to_string(),
                },
                |r, _| {
                    r.read_vector(|r| -> Value {
                        r.get_save_map_namedspaced_object().into()
                    })
                    .into()
                },
            )
            .into(),
        );

        map.into()
    }
}
