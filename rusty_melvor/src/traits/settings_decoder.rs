use serde_json::{Map, Value};

use super::read::DataReaders;

macro_rules! madd {
    ($map:expr, $key:expr, $reader:expr) => {
        $map.insert($key.into(), $reader.into());
    };
}

pub trait SettingsDecoder: DataReaders {
    fn decode_settings(&mut self) -> Value {
        let r = self;
        let mut map = Map::new();

        let bool_settings: &[&str] = &[
            "continue_if_bank_full",
            "continue_thieving_on_stun",
            "auto_restart_dungeon",
            "auto_cloud_save",
            "dark_mode",
            "show_gp_notifications",
            "enable_accessibility",
            "show_enemy_skill_levels",
            "show_close_confirmations",
            "hide_thousands_seperator",
            "show_virtual_levels",
            "show_sale_confirmations",
            "show_shop_confirmations",
            "pause_on_unfocus",
            "show_combat_minibar",
            "show_combat_minibar_combat",
            "show_skilling_minibar",
            "use_combination_runes",
            "enable_auto_slayer",
            "show_item_notifications",
            "use_small_level_up_notifications",
            "use_default_bank_borders",
            "default_to_current_equip_set",
            "hide_max_level_masteries",
            "show_mastery_checkpointconfirmations",
            "enable_offline_push_notifications",
            "enable_farming_push_notifications",
            "enable_offline_combat",
            "enable_mini_sidebar",
            "enable_auto_equip_food",
            "enable_auto_swap_food",
            "enable_perfect_cooking",
            "show_crop_destruction_confirmations",
            "show_astrology_max_roll_confirmations",
            "show_quantity_in_item_notifications",
            "show_item_preservation_notifications",
            "show_slayer_coin_notifications",
            "show_equipment_sets_in_combat_minibar",
            "show_bars_in_combat_minibar",
            "show_combat_stun_notifications",
            "show_combat_sleep_notifications",
            "show_summoning_mark_discovery_modals",
            "enable_combat_damage_splashes",
            "enable_progress_bars",
            "show_tier_i_potions",
            "show_tier_ii_potions",
            "show_tier_iii_potions",
            "show_tier_iv_potions",
        ];

        for setting in bool_settings {
            madd!(map, setting.to_string(), r.read_bool());
        }

        madd!(map, "show_neutral_attack_modifiers", r.read_bool());
        madd!(map, "default_page_on_load", r.read_namespaced_object());

        madd!(map, "format_number_setting", r.read_uint8());
        madd!(map, "bank_sort_order", r.read_uint8());
        madd!(map, "colour_blind_mode", r.read_uint8());

        madd!(map, "enableEyebleachMode", r.read_bool());

        madd!(map, "enableQuickConvert", r.read_bool());

        madd!(map, "show_locked_township_buildings", r.read_bool());

        madd!(map, "use_new_notifications", r.read_bool());
        madd!(map, "notification_horizontal_position", r.read_uint8());
        madd!(map, "notification_disappear_delay", r.read_uint8());
        madd!(map, "showItemNamesInNotifications", r.read_bool());

        madd!(map, "importanceSummoningMarkFound", r.read_bool());
        madd!(map, "importanceErrorMessages", r.read_bool());

        madd!(map, "enableScrollableBankTabs", r.read_bool());

        madd!(map, "showWikiLinks", r.read_bool());

        madd!(map, "disableHexGridOutsideSight", r.read_bool());

        madd!(map, "mapTextureQuality", r.read_uint8());
        madd!(map, "enableMapAntialiasing", r.read_bool());

        madd!(map, "show_skill_xp_notifications", r.read_bool());
        madd!(map, "background_image", r.read_uint8());
        madd!(map, "super_dark_mode", r.read_bool());
        madd!(map, "show_expansion_background_colours", r.read_bool());
        madd!(map, "show_combat_area_warnings", r.read_bool());
        madd!(map, "use_compact_notifications", r.read_bool());
        madd!(map, "use_legacy_notifications", r.read_bool());
        madd!(map, "use_cat", r.read_bool());
        madd!(map, "throttle_frame_rate_on_inactivity", r.read_bool());
        madd!(map, "cartography_frame_rate_cap", r.read_uint16());
        madd!(map, "toggle_birthday_event", r.read_bool());
        madd!(map, "toggle_discord_rpc", r.read_bool());
        madd!(map, "generic_artefact_all_but_one", r.read_bool());

        madd!(
            map,
            "hidden_mastery_namespaces",
            r.read_set(|r| r.read_string())
        );

        madd!(map, "enable_double_click_equip", r.read_bool());
        madd!(map, "enable_double_click_open", r.read_bool());
        madd!(map, "enable_double_click_bury", r.read_bool());
        madd!(map, "show_abyssal_pieces_notifications", r.read_bool());
        madd!(map, "show_abyssal_slayer_coin_notifications", r.read_bool());
        madd!(map, "enable_perma_corruption", r.read_bool());
        madd!(map, "show_ap_next_to_shop_sidebar", r.read_bool());
        madd!(map, "show_asc_next_to_slayer_sidebar", r.read_bool());
        madd!(map, "sidebar_levels", r.read_uint8());
        madd!(map, "show_abyssal_xp_notifications", r.read_bool());
        madd!(map, "show_sp_next_to_prayer_sidebar", r.read_bool());
        madd!(map, "enable_sticky_bank_tabs", r.read_bool());
        madd!(map, "use_legacy_realm_selection", r.read_bool());
        madd!(map, "show_opacity_for_skill_navs", r.read_bool());
        madd!(map, "bank_filter_show_all", r.read_bool());
        madd!(map, "bank_filter_show_demo", r.read_bool());
        madd!(map, "bank_filter_show_full", r.read_bool());
        madd!(map, "bank_filter_show_tot_h", r.read_bool());
        madd!(map, "bank_filter_show_ao_d", r.read_bool());
        madd!(map, "bank_filter_show_it_a", r.read_bool());
        madd!(map, "bank_filter_show_damage_reduction", r.read_bool());
        madd!(map, "bank_filter_show_abyssal_resistance", r.read_bool());
        madd!(map, "bank_filter_show_normal_damage", r.read_bool());
        madd!(map, "bank_filter_show_abyssal_damage", r.read_bool());
        madd!(map, "bank_filter_show_skill_xp", r.read_bool());
        madd!(map, "bank_filter_show_abyssal_xp", r.read_bool());
        madd!(map, "always_show_realm_select_agility", r.read_bool());
        madd!(map, "enable_swipe_sidebar", r.read_bool());

        map.into()
    }
}
