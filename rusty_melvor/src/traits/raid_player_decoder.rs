use serde_json::{Map, Value};

use crate::NamespacedObject;

use super::player_decoder::PlayerDecoder;

pub trait RaidPlayerDecoder: PlayerDecoder {
    fn decode_raid_player(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert("player".into(), r.decode_player());
        map.insert(
            "alt_attacks".into(),
            r.read_value_map_key(
                |r| match r.get_save_map_namedspaced_object() {
                    NamespacedObject {
                        text_id: Some(text_id),
                        ..
                    } => text_id,
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
