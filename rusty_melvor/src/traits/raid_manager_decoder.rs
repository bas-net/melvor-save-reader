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
            "base_manager".into(),
            r.decode_base_manager(
                |r| r.decode_raid_player(),
                |r| r.decode_raid_enemy(),
            ),
        );

        // println!("size: {}", r.read_uint32());
        // println!("modifier_values: {:?}", decode_modifier_values(r));

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

        map.insert("wave".into(), r.read_uint32().into());
        map.insert("wave_progress".into(), r.read_uint32().into());
        map.insert("kill_count".into(), r.read_uint32().into());

        map.insert("start_timestamp".into(), r.read_float64().into());

        map.insert(
            "owned_crate_items".into(),
            r.read_set(|r| -> Value {
                r.get_save_map_namedspaced_object().into()
            })
            .into(),
        );

        map.insert(
            "random_modifiers_being_selected".into(),
            r.read_vector(|r| -> Value { decode_modifier_values(r) })
                .into(),
        );

        map.insert(
            "is_selecting_positive_modifier".into(),
            r.read_bool().into(),
        );

        map.insert("items_being_selected".into(), {
            let mut map = Map::new();
            map.insert(
                "weapons".into(),
                r.read_vector(|r| -> Value { get_selection(r) }).into(),
            );
            map.insert(
                "armour".into(),
                r.read_vector(|r| -> Value { get_selection(r) }).into(),
            );
            map.insert(
                "ammo".into(),
                r.read_vector(|r| -> Value { get_selection(r) }).into(),
            );
            map.insert(
                "runes".into(),
                r.read_vector(|r| -> Value { get_selection(r) }).into(),
            );
            map.insert(
                "food".into(),
                r.read_vector(|r| -> Value { get_selection(r) }).into(),
            );
            map.insert(
                "passives".into(),
                r.read_vector(|r| -> Value { get_selection(r) }).into(),
            );

            map.into()
        });

        map.insert(
            "item_category_being_selected".into(),
            {
                match r.read_uint8() {
                    0 => "weapons",
                    1 => "armour",
                    2 => "ammo",
                    3 => "runes",
                    4 => "food",
                    5 => "passives",
                    _ => "other",
                }
            }
            .into(),
        );

        map.insert("pos_mods_selected".into(), r.read_uint8().into());
        map.insert("neg_mods_selected".into(), r.read_uint8().into());

        map.insert("is_paused".into(), r.read_bool().into());

        // history
        map.insert(
            "history".into(),
            r.read_vector(|r| {
                let mut map = Map::new();
                map.insert(
                    "skill_levels".into(),
                    r.read_vector(|r| -> Value { r.read_uint32().into() })
                        .into(),
                );
                map.insert(
                    "equipment".into(),
                    r.read_vector(|r| -> Value {
                        r.get_save_map_namedspaced_object().into()
                    })
                    .into(),
                );
                map.insert("ammo".into(), r.read_uint32().into());
                map.insert(
                    "inventory".into(),
                    r.read_vector(|r| -> Value {
                        let mut map = Map::new();
                        map.insert(
                            "item".into(),
                            r.get_save_map_namedspaced_object().into(),
                        );
                        map.insert("quantity".into(), r.read_uint32().into());
                        map.into()
                    })
                    .into(),
                );
                map.insert("food".into(), {
                    let mut map = Map::new();
                    map.insert(
                        "item".into(),
                        r.get_save_map_namedspaced_object().into(),
                    );
                    map.insert("quantity".into(), r.read_uint32().into());
                    Into::<Value>::into(map)
                });

                map.insert("wave".into(), r.read_uint32().into());
                map.insert("kills".into(), r.read_uint32().into());
                map.insert("timestamp".into(), r.read_float64().into());
                map.insert("raid_coins_earned".into(), r.read_uint32().into());
                map.insert("difficulty".into(), r.read_uint8().into());

                map
            })
            .into(),
        );

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

fn get_selection<T: RaidManagerDecoder>(r: &mut T) -> Value {
    let mut map = Map::new();
    map.insert("item".into(), r.get_save_map_namedspaced_object().into());
    map.insert("quantity".into(), r.read_uint32().into());
    map.insert("isAlt".into(), r.read_bool().into());
    map.into()
}
