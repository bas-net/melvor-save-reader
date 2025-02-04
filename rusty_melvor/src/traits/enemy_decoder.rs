use serde_json::{Map, Value};

use super::character_decoder::CharacterDecoder;

pub trait EnemyDecoder: CharacterDecoder {
    fn decode_enemy(&mut self, encode_monster: bool) -> Value {
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

        if encode_monster && r.read_bool() {
            map.insert(
                "monster".into(),
                r.read_namespaced_object().into(),
            );
        }

        if r.read_bool() {
            map.insert(
                "override_damage_type".into(),
                r.read_namespaced_object().into(),
            );
        }

        map.into()
    }
}
