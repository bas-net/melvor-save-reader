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
                |r| match r.read_namespaced_object() {
                    NamespacedObject {
                        text_id: Some(text_id),
                        ..
                    } => text_id,
                    NamespacedObject { id, .. } => id.to_string(),
                },
                |r, _| {
                    r.read_vector(|r| -> Value {
                        r.read_namespaced_object().into()
                    })
                    .into()
                },
            )
            .into(),
        );

        map.into()
    }
}
