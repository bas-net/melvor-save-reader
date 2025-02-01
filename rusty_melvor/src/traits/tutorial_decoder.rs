use serde_json::{Map, Value};

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait TutorialDecoder: DataReaders {
    fn decode_tutorial(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        let complete = r.read_bool();
        madd!(map, "complete", complete);

        if !complete {
            if r.read_bool() {
                madd!(map, "current_stage", r.read_namespaced_object());

                madd!(map, "current_stage_claimed", r.read_bool());

                let num_tasks = r.read_uint32();

                madd!(map, "tasks", {
                    let mut tasks = Vec::<Map<String, Value>>::new();
                    for i in 0..num_tasks {
                        tasks.push({
                            let mut task = Map::new();
                            madd!(task, "task_id", i);
                            madd!(task, "progress", r.read_uint8());
                            task.into()
                        });
                    }
                    tasks
                });
            }
        }

        map.into()
    }
}
