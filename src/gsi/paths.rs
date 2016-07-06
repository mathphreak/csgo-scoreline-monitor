use super::super::prefs::Prefs;

pub const CFG_PREFIX: &'static str = "gamestate_integration_csgo-scoreline-monitor_";

pub fn get_csgo_cfg() -> String {
    let path = Prefs::get().unwrap_or(Default::default()).csgo_cfg_path;
    path
}
