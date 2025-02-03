use serde_json::{Map, Value};

use super::{read::DataReaders, timer_decoder::TimerDecoder};

pub trait BaseManagerDecoder: DataReaders + TimerDecoder {
    fn decode_base_manager(
        &mut self,
        call_decode_player: fn(&mut Self) -> Value,
        call_decode_enemy: fn(&mut Self) -> Value,
    ) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert("player".into(), call_decode_player(r));
        map.insert("enemy".into(), call_decode_enemy(r));

        map.insert("fight_in_progress".into(), r.read_bool().into());
        map.insert("spawn_timer".into(), r.decode_timer());
        map.insert("is_active".into(), r.read_bool().into());

        // decode passives
        map.insert(
            "passives".into(),
            r.read_vector(|r| -> Value {
                let passive = r.get_save_map_namedspaced_object();
                let mut passive_map = Map::new();
                passive_map.insert("passive".into(), passive.into());
                passive_map.insert("display".into(), r.read_bool().into());
                passive_map.into()
            })
            .into(),
        );

        map.into()
    }
}
