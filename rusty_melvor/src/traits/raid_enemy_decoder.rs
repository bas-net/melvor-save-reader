use serde_json::{Map, Value};

use super::enemy_decoder::EnemyDecoder;

pub trait RaidEnemyDecoder: EnemyDecoder {
    fn decode_raid_enemy(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        map.insert("enemy".into(), r.decode_enemy(false));

        // r.print_previous_bytes(100);
        // r.print_next_bytes(100);

        if r.read_bool() {
            map.insert(
                "monster".into(),
                {
                    let mut map = Map::new();
                    map.insert("name".into(), r.read_string().into());
                    map.insert(
                        "levels".into(),
                        {
                            let mut map = Map::new();
                            map.insert(
                                "hitpoints".into(),
                                r.read_uint32().into(),
                            );
                            map.insert(
                                "attack".into(),
                                r.read_uint32().into(),
                            );
                            map.insert(
                                "strength".into(),
                                r.read_uint32().into(),
                            );
                            map.insert(
                                "defence".into(),
                                r.read_uint32().into(),
                            );
                            map.insert(
                                "ranged".into(),
                                r.read_uint32().into(),
                            );
                            map.insert("magic".into(), r.read_uint32().into());

                            map
                        }
                        .into(),
                    );
                    map.insert("attack_type".into(), r.read_uint8().into());
                    map.insert("golbin-svg".into(), r.read_uint8().into());
                    map.insert(
                        "passives".into(),
                        r.read_vector(|r| -> Value {
                            r.get_save_map_namedspaced_object().into()
                        })
                        .into(),
                    );

                    map.insert("corruption".into(), r.read_uint32().into());

                    map
                }
                .into(),
            );
        }

        map.into()
    }
}
