use serde_json::{Map, Value};

use crate::{IntoValue, NamespacedObject};

use super::{
    base_manager_decoder::BaseManagerDecoder, player_decoder::PlayerDecoder,
    raid_enemy_decoder::RaidEnemyDecoder,
    raid_player_decoder::RaidPlayerDecoder, read::DataReaders,
    timer_decoder::TimerDecoder,
};

pub trait BankDecoder: DataReaders {
    fn decode_bank(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert(
            "locked_items".into(),
            r.read_set(|r| -> Value {
                r.get_save_map_namedspaced_object().into()
            })
            .into(),
        );

        map.insert(
            "items_by_bank_tab".into(),
            r.read_vector(|r| -> Value {
                r.read_vector(|r| {
                    let item = r.get_save_map_namedspaced_object();
                    BankItem {
                        item: item,
                        quantity: r.read_uint32(),
                    }
                    .into_value()
                }).into()
            })
            .into(),
        );

        map.insert(
            "default_item_tabs".into(),
            r.read_value_map_key(
                |r| {
                    let item = r.get_save_map_namedspaced_object();
                    match item.text_id {
                        Some(text_id) => text_id,
                        None => item.id.to_string(),
                    }
                },
                |r, _| r.read_uint8().into(),
            )
            .into(),
        );

        // save_reader.add_to_save_map("custom_sort_order", |r| {
        //     r.read_vector(|r| {
        //         let item = r.get_save_map_namedspaced_object();
        //         item
        //     })
        // });
        map.insert(
            "custom_sort_order".into(),
            r.read_vector(|r| -> Value {
                r.get_save_map_namedspaced_object().into()
            })
            .into(),
        );

        // save_reader.add_to_save_map("glowing_items", |r| {
        //     r.read_set(|r| r.get_save_map_namedspaced_object())
        // });
        map.insert(
            "glowing_items".into(),
            r.read_set(|r| -> Value {
                r.get_save_map_namedspaced_object().into()
            })
            .into(),
        );

        // save_reader.add_to_save_map("tab_icons", |r| -> Value {
        //     r.read_value_map_key(
        //         |r| r.read_uint8().to_string(),
        //         |r, _| r.get_save_map_namedspaced_object().into(),
        //     )
        //     .into()
        // });
        map.insert(
            "tab_icons".into(),
            r.read_value_map_key(
                |r| r.read_uint8().to_string(),
                |r, _| r.get_save_map_namedspaced_object().into(),
            )
            .into(),
        );

        Value::Object(map)
    }
}

struct BankItem {
    item: NamespacedObject,
    quantity: u32,
}

impl IntoValue for BankItem {
    fn into_value(self) -> Value {
        let mut map = Map::new();
        map.insert("item".into(), self.item.into());
        map.insert("quantity".into(), self.quantity.into());
        Value::Object(map)
    }
}
