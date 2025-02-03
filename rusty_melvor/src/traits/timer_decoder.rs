use serde_json::{Map, Value};

use super::read::DataReaders;

pub trait TimerDecoder: DataReaders {
    fn decode_timer(&mut self) -> Value {
        let r = self;

        let mut map = Map::new();
        map.insert("ticksleft".into(), r.read_uint32().into());
        map.insert("maxticks".into(), r.read_uint32().into());
        map.insert("active".into(), r.read_bool().into());
        map.into()
    }
}
