use serde_json::{Map, Value};

use crate::NamespacedObject;

use super::{
    character_decoder::CharacterDecoder, player_decoder::PlayerDecoder,
};

pub trait EnemyDecoder: CharacterDecoder {
    fn decode_enemy(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert("character".into(), r.decode_character());

        map.insert("state".into(), r.read_uint8().into());
        map.insert("random_attack_type".into(), {
            match r.read_uint8() {
                0 => "Melee",
                1 => "Ranged",
                2 => "Magic",
                3 => "Unset",
                _ => "Other",
            }
            .into()
        });

        if r.read_bool() {
            map.insert(
                "monster".into(),
                r.get_save_map_namedspaced_object().into(),
            );
        }

        if r.read_bool() {
            map.insert(
                "override_damage_type".into(),
                r.get_save_map_namedspaced_object().into(),
            );
        }

        map.into()
    }
}
