use serde_json::{Map, Value};

use super::{read::DataReaders, timer_decoder::TimerDecoder};

pub trait CharacterDecoder: DataReaders + TimerDecoder {
    fn decode_character(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();
        map.insert("combat_manager.hitpoints".into(), r.read_uint32().into());
        map.insert(
            "nextAction".into(),
            {
                match r.read_uint8() {
                    1 => "Attack",
                    _ => "Nothing",
                }
            }
            .into(),
        );

        map.insert("attack_count".into(), r.read_uint32().into());
        map.insert(
            "next_attack".into(),
            r.get_save_map_namedspaced_object().into(),
        );
        map.insert("is_attacking".into(), r.read_bool().into());
        map.insert("first_hit".into(), r.read_bool().into());

        map.insert("timers_act".into(), r.decode_timer());
        map.insert("timers_regen".into(), r.decode_timer());

        map.insert("turns_taken".into(), r.read_uint32().into());
        map.insert("buffered_regen".into(), r.read_uint32().into());
        // decode active effects
        map.insert(
            "active_effects".into(),
            r.read_value_map_key(
                |r| match r.get_save_map_namedspaced_object().text_id {
                    Some(text_id) => text_id,
                    None => "".to_string(),
                },
                |r, k| {
                    if k.is_empty() {
                        // Skip
                        r.skip(18);
                        r.read_vector(|r| {
                            r.read_string();
                            r.skip(4);
                            Value::Null
                        });
                        r.read_vector(|r| {
                            r.read_string();
                            r.skip(4);
                            Value::Null
                        });
                        r.read_vector(|r| {
                            r.read_string();
                            r.skip(4);
                            Value::Null
                        });
                        r.read_vector(|r| {
                            r.read_string();
                            // Skip timer
                            r.skip(9);
                            Value::Null
                        });

                        Value::Null
                    } else {
                        // ActiveEffect decode
                        let mut map = Map::new();
                        map.insert(
                            "source_character".into(),
                            r.read_bool().into(),
                        );

                        let source = r.read_uint8();
                        map.insert(
                            "source".into(),
                            match source {
                                0 => "Attack".into(),
                                1 => "Effect".into(),
                                _ => "Other".into(),
                            },
                        );

                        map.insert(
                            "damageDealt".into(),
                            r.read_float64().into(),
                        );
                        map.insert(
                            "damageTaken".into(),
                            r.read_float64().into(),
                        );

                        map.insert(
                            "parameters".into(),
                            r.read_vector(|r| -> Value {
                                let mut param_map = Map::new();
                                param_map.insert(
                                    "name".into(),
                                    r.read_string().into(),
                                );
                                param_map.insert(
                                    "value".into(),
                                    r.read_uint32().into(),
                                );
                                param_map.into()
                            })
                            .into(),
                        );

                        map.insert(
                            "stat_groups".into(),
                            r.read_vector(|r| -> Value {
                                let mut map = Map::new();
                                map.insert(
                                    "name".into(),
                                    r.read_string().into(),
                                );
                                map.insert(
                                    "value".into(),
                                    r.read_uint32().into(),
                                );
                                map.into()
                            })
                            .into(),
                        );

                        map.insert(
                            "timers".into(),
                            r.read_vector(|r| -> Value {
                                let mut map = Map::new();
                                map.insert(
                                    "name".into(),
                                    r.read_string().into(),
                                );
                                map.insert("timer".into(), r.decode_timer());
                                map.into()
                            })
                            .into(),
                        );

                        map.into()
                    }
                },
            )
            .into(),
        );

        map.insert("first_miss".into(), r.read_bool().into());
        map.insert("barrier".into(), r.read_uint32().into());

        map.into()
    }
}
