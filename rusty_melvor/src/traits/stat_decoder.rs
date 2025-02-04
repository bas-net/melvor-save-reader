use serde_json::{Map, Value};

use crate::NamespacedObject;

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

struct Skill {
    name: &'static str,
    stats: &'static [&'static str],
}

pub trait StatDecoder: DataReaders {
    fn decode_stats(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        let skill_maps = [
            Skill {
                name: "woodcutting",
                stats: &["Actions", "TimeSpent", "LogsCut", "BirdNestsGotten"],
            },
            Skill {
                name: "fishing",
                stats: &[
                    "FishCaught",
                    "JunkCaught",
                    "SpecialItemsCaught",
                    "TimeSpent",
                    "Actions",
                    "GPEarned",
                ],
            },
            Skill {
                name: "firemaking",
                stats: &[
                    "LogsBurnt",
                    "GPBurnt",
                    "TimeSpent",
                    "BonusBonfireXP",
                    "TotalActions",
                    "BonfiresLit",
                    "ItemsPreserved",
                    "GPEarned",
                    "CoalGained",
                ],
            },
            Skill {
                name: "cooking",
                stats: &[
                    "FoodCooked",
                    "FoodBurnt",
                    "TimeSpent",
                    "SuccessfulActions",
                    "PerfectCooks",
                    "PassiveCooks",
                    "ItemsUsed",
                    "ItemsPreserved",
                ],
            },
            Skill {
                name: "mining",
                stats: &[
                    "Actions",
                    "EmptyOresMined",
                    "TimeSpent",
                    "OresGained",
                    "GemsGained",
                    "RockHPPreserved",
                    "RocksDepleted",
                    "OnyxGemNodesFound",
                    "TotalOnyxGemNodeHPFound",
                    "OrichaGemNodesFound",
                    "TotalOrichaGemNodeHPFound",
                    "CeruleanGemNodesFound",
                    "TotalCeruleanGemNodeHPFound",
                    "NightopalGemNodesFound",
                    "TotalNightopalGemNodeHPFound",
                    "ShadowpearlGemNodesFound",
                    "TotalShadowpearlGemNodeHPFound",
                    "MoonstoneGemNodesFound",
                    "TotalMoonstoneGemNodeHPFound",
                    "VoidheartGemNodesFound",
                    "TotalVoidheartGemNodeHPFound",
                ],
            },
            Skill {
                name: "smithing",
                stats: &[
                    "SmeltingActions",
                    "SmithingActions",
                    "TimeSpent",
                    "BarsUsed",
                    "BarsPreserved",
                    "OresUsed",
                    "OresPreserved",
                    "TotalItemsSmithed",
                    "TotalBarsSmelted",
                ],
            },
            Skill {
                name: "attack",
                stats: &[],
            },
            Skill {
                name: "strength",
                stats: &[],
            },
            Skill {
                name: "defence",
                stats: &[],
            },
            Skill {
                name: "hitpoints",
                stats: &[],
            },
            Skill {
                name: "thieving",
                stats: &[
                    "SuccessfulPickpockets",
                    "FailedPickpockets",
                    "DamageTakenFromNPCs",
                    "TimeSpentStunned",
                    "TimeSpent",
                    "GPStolen",
                    "CommonDrops",
                    "RareDrops",
                    "AreaDrops",
                    "NpcDrops",
                ],
            },
            Skill {
                name: "farming",
                stats: &[
                    "AllotmentsHarvested",
                    "CompostUsed",
                    "CropsDied",
                    "TimeSpentWaitingForCrops",
                    "TimeSpentWaitingForDeadCrops",
                    "GloopUsed",
                    "HerbsHarvested",
                    "TreesHarvested",
                    "SeedsPlanted",
                    "HerbsGained",
                    "LogsGained",
                    "FoodGained",
                ],
            },
            Skill {
                name: "ranged",
                stats: &[],
            },
            Skill {
                name: "fletching",
                stats: &[
                    "ArrowShaftsMade",
                    "ItemsFletched",
                    "TimeSpent",
                    "ItemsUsed",
                    "ItemsPreserved",
                    "Actions",
                ],
            },
            Skill {
                name: "crafting",
                stats: &[
                    "ItemsCrafted",
                    "TimeSpent",
                    "ItemsUsed",
                    "ItemsPreserved",
                    "Actions",
                    "GPUsed",
                    "GPPreserved",
                ],
            },
            Skill {
                name: "runecrafting",
                stats: &[
                    "ItemsCrafted",
                    "TimeSpent",
                    "ItemsUsed",
                    "ItemsPreserved",
                    "Actions",
                ],
            },
            Skill {
                name: "magic",
                stats: &[],
            },
            Skill {
                name: "prayer",
                stats: &[
                    "BonesBuried",
                    "PrayerPointsEarned",
                    "PrayerPointsSpent",
                    "PrayerPointsPreserved",
                    "SoulsReleased",
                    "SoulPointsEarned",
                    "SoulPointsSpent",
                    "SoulPointsPreserved",
                ],
            },
            Skill {
                name: "slayer",
                stats: &["SlayerCoinsEarned", "MonstersKilledOnTask"],
            },
            Skill {
                name: "herblore",
                stats: &[
                    "PotionsMade",
                    "TimeSpent",
                    "PotionsUsed",
                    "ChargesUsed",
                    "ItemsUsed",
                    "ItemsPreserved",
                    "Actions",
                ],
            },
            Skill {
                name: "agility",
                stats: &[
                    "ObstaclesCompleted",
                    "CoursesCompleted",
                    "GPEarned",
                    "TimeSpent",
                    "SlayerCoinsEarned",
                    "ItemsEarned",
                ],
            },
            Skill {
                name: "summoning",
                stats: &[
                    "Actions",
                    "TimeSpent",
                    "ItemsMade",
                    "ItemsUsed",
                    "ItemsPreserved",
                    "GPUsed",
                    "GPPreserved",
                    "SCUsed",
                    "SCPreserved",
                    "TabletsUsed",
                ],
            },
            Skill {
                name: "items",
                stats: &[
                    "TimesFound",
                    "TimesSold",
                    "GpFromSale",
                    "TimesLostToDeath",
                    "DamageTaken",
                    "DamageDealt",
                    "MissedAttacks",
                    "TimesEaten",
                    "HealedFor",
                    "TotalAttacks",
                    "AmountUsedInCombat",
                    "TimeWaited",
                    "TimesDied",
                    "TimesGrown",
                    "HarvestAmount",
                    "EnemiesKilled",
                    "TimesOpened",
                    "TimesTransformed",
                    "TimesBuried",
                    "TimesReleased",
                ],
            },
            Skill {
                name: "monster",
                stats: &[
                    "DamageDealtToPlayer",
                    "DamageTakenFromPlayer",
                    "KilledByPlayer",
                    "KilledPlayer",
                    "HitsToPlayer",
                    "HitsFromPlayer",
                    "EnemyMissed",
                    "PlayerMissed",
                    "Seen",
                    "RanAway",
                ],
            },
            Skill {
                name: "general",
                stats: &[
                    "TotalGPEarned",
                    "TotalItemsSold",
                    "UsernameChanges",
                    "AccountCreationDate",
                    "SignetRingHalvesMissed",
                ],
            },
            Skill {
                name: "combat",
                stats: &[
                    "MonstersKilled",
                    "DamageDealt",
                    "DamageTaken",
                    "AttacksMissed",
                    "Deaths",
                    "FoodConsumed",
                    "HPFromFood",
                    "TimeSpentSpawning",
                    "TimeSpentFighting",
                    "TimeSpentPaused",
                    "ItemsLooted",
                    "GPEarned",
                    "DungeonRewards",
                ],
            },
            Skill {
                name: "raid",
                stats: &[
                    "GolbinsKilled",
                    "HighestWave",
                    "RaidCoinsEarned",
                    "TotalTimeSpent",
                    "LongestRaid",
                    "TotalDeath",
                    "WavesCompleted",
                ],
            },
            Skill {
                name: "astrology",
                stats: &[
                    "TimeSpent",
                    "StandardRerolls",
                    "UniqueRerolls",
                    "MaxRollsHit",
                    "MinRollsHit",
                    "Actions",
                    "MeteoritesLocated",
                    "TotalMeteoriteHP",
                    "AbyciteLocated",
                    "TotalAbyciteHP",
                    "MysticiteLoated",
                    "TotalMysticiteHP",
                    "EchociteLocated",
                    "TotalEchociteHP",
                ],
            },
            Skill {
                name: "shop",
                stats: &[
                    "PurchasesMade",
                    "ItemsPurchased",
                    "GPSpent",
                    "SCSpent",
                    "RCSpent",
                    "ItemsSpent",
                    "GloveChargesPurchased",
                ],
            },
            Skill {
                name: "township",
                stats: &[],
            },
            Skill {
                name: "cartography",
                stats: &[],
            },
            Skill {
                name: "archaeology",
                stats: &[],
            },
            Skill {
                name: "corruption",
                stats: &[],
            },
            Skill {
                name: "harvesting",
                stats: &[],
            },
        ];

        for skill in skill_maps {
            if ["items", "monster"].contains(&skill.name) {
                madd!(
                    map,
                    skill.name,
                    r.decode_mapped_stat_tracker(skill.stats)
                );
            } else {
                madd!(map, skill.name, r.decode_stat_tracker(skill.stats));
            }
        }

        map.into()
    }

    fn decode_stat_tracker(&mut self, name_map: &[&str]) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(
            map,
            "stats",
            r.read_value_map_key(
                |r| {
                    let id = r.read_uint32();
                    match name_map.get(id as usize) {
                        Some(name) => name.to_string(),
                        None => id.to_string(),
                    }
                }
                .into(),
                |r, _| r.read_float64().into()
            )
        );

        map.into()
    }

    fn decode_mapped_stat_tracker(&mut self, name_map: &[&str]) -> Value {
        let r = self;
        let mut map = Map::new();

        madd!(
            map,
            "stats_map",
            r.read_value_map_key(
                |r| match r.read_namespaced_object() {
                    NamespacedObject {
                        text_id: Some(text_id),
                        ..
                    } => text_id,
                    NamespacedObject { id, .. } => id.to_string(),
                },
                |r, _| r.decode_stat_tracker(name_map).into()
            )
        );

        map.into()
    }
}
