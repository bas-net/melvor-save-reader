use serde_json::{Map, Value};

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait NativeManagerDecoder: DataReaders {
    fn decode_native_manager(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(
            map,
            "scheduled_push_notifications",
            r.read_vector(|r| -> Value {
                let mut map = Map::new();
                madd!(map, "id", r.read_string());
                madd!(map, "startDate", r.read_float64());
                madd!(map, "endDate", r.read_float64());
                madd!(map, "notificationType", r.read_uint8());
                madd!(map, "platform", r.read_string());

                map.into()
            })
        );

        map.into()
    }
}
