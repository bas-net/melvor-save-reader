use serde_json::{Map, Value};

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait StatDecoder: DataReaders {
    fn decode_stats(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        // WoodcuttingStats[WoodcuttingStats["Actions"] = 0] = "Actions";
        // WoodcuttingStats[WoodcuttingStats["TimeSpent"] = 1] = "TimeSpent";
        // WoodcuttingStats[WoodcuttingStats["LogsCut"] = 2] = "LogsCut";
        // WoodcuttingStats[WoodcuttingStats["BirdNestsGotten"] = 3] = "BirdNestsGotten";

        madd!(map, "woodcutting", r.decode_stat_tracker());
        madd!(map, "fishing", r.decode_stat_tracker());

        map.into()
    }

    fn decode_stat_tracker(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(
            map,
            "stats",
            r.read_value_map_key(
                |r| r.read_uint32().to_string().into(),
                |r, _| r.read_float64().into()
            )
        );

        map.into()
    }
}
