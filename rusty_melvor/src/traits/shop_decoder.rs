use serde_json::{Map, Value};

use super::read::DataReaders;

pub trait ShopDecoder: DataReaders {
    fn decode_shop(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert(
            "upgrades_purchased".into(),
            r.read_value_map_key(
                |r| {
                    let item = r.get_save_map_namedspaced_object();
                    match item.text_id {
                        Some(text_id) => text_id,
                        None => item.id.to_string(),
                    }
                },
                |r, _| r.read_uint32().into(),
            )
            .into(),
        );

        map.insert("buy_quantity".into(), r.read_float64().into());

        map.into()
    }
}
