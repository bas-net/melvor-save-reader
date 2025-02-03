use serde_json::{Map, Value};

use super::{
    bank_decoder::BankDecoder, base_manager_decoder::BaseManagerDecoder,
    raid_enemy_decoder::RaidEnemyDecoder,
    raid_player_decoder::RaidPlayerDecoder,
};

enum ModifierScope {
    Skill = 1,
    DamageType = 2,
    Realm = 4,
    Currency = 8,
    Category = 16,
    Action = 32,
    Subcategory = 64,
    Item = 128,
    EffectGroup = 256,
}

pub trait RaidManagerDecoder:
    BaseManagerDecoder + RaidEnemyDecoder + RaidPlayerDecoder + Sized + BankDecoder
{
    fn decode_raid_manager(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert(
            "raid_player".into(),
            r.decode_base_manager(
                |r| r.decode_raid_player(),
                |r| r.decode_raid_enemy(),
            ),
        );

        map.insert(
            "random_player_modifiers".into(),
            r.read_vector(|r| -> Value { decode_modifier_values(r) })
                .into(),
        );
        map.insert(
            "random_enemy_modifiers".into(),
            r.read_vector(|r| -> Value { decode_modifier_values(r) })
                .into(),
        );

        map.insert("state".into(), r.read_uint8().into());
        map.insert("_set_dificulty".into(), r.read_uint8().into());

        // GolbinRaidBank
        map.insert("bank".into(), r.decode_bank());

        map.into()
    }
}

fn decode_modifier_values<T: RaidManagerDecoder>(r: &mut T) -> Value {
    let mut map = Map::new();
    map.insert(
        "modifier".into(),
        r.get_save_map_namedspaced_object().into(),
    );
    map.insert("value".into(), r.read_float64().into());
    let scope_key = r.read_uint32();
    map.insert("scope_key".into(), scope_key.into());

    if scope_key & ModifierScope::Skill as u32 != 0 {
        map.insert("skill".into(), r.get_save_map_namedspaced_object().into());
    }
    if scope_key & ModifierScope::DamageType as u32 != 0 {
        map.insert(
            "damage_type".into(),
            r.get_save_map_namedspaced_object().into(),
        );
    }

    if scope_key & ModifierScope::Realm as u32 != 0 {
        map.insert("realm".into(), r.get_save_map_namedspaced_object().into());
    }

    if scope_key & ModifierScope::Currency as u32 != 0 {
        map.insert(
            "currency".into(),
            r.get_save_map_namedspaced_object().into(),
        );
    }

    if scope_key & ModifierScope::Category as u32 != 0 {
        map.insert(
            "category".into(),
            r.get_save_map_namedspaced_object().into(),
        );
    }

    if scope_key & ModifierScope::Action as u32 != 0 {
        map.insert(
            "action".into(),
            r.get_save_map_namedspaced_object().into(),
        );
    }

    if scope_key & ModifierScope::Subcategory as u32 != 0 {
        map.insert(
            "subcategory".into(),
            r.get_save_map_namedspaced_object().into(),
        );
    }

    if scope_key & ModifierScope::Item as u32 != 0 {
        map.insert("item".into(), r.get_save_map_namedspaced_object().into());
    }

    if scope_key & ModifierScope::EffectGroup as u32 != 0 {
        map.insert(
            "effect_group".into(),
            r.get_save_map_namedspaced_object().into(),
        );
    }

    map.into()
}
