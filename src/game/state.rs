extern crate time;

use super::super::gsi::{self, message};
use super::super::gsi::message::{TakesUpdates, UpdateReason};
use super::super::prefs::Prefs;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct Scores {
    ct: u8,
    t: u8,
}

impl ToJson for Scores {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("CT".to_string(), self.ct.to_json());
        d.insert("T".to_string(), self.t.to_json());
        Json::Object(d)
    }
}

pub struct State {
    last_up: time::Tm,
    in_game: bool,
    team: Option<message::Team>,
    gsi: gsi::Versions,
    gamemode: message::Mode,
    scores: Scores,
    round_start: Option<time::Tm>,
    round_live: bool,
    bomb_down: bool,
    map: String,
}

impl Default for State {
    fn default() -> State {
        State {
            last_up: time::at(time::Timespec::new(0, 0)),
            in_game: false,
            team: None,
            gsi: gsi::Versions::new(),
            gamemode: Default::default(),
            scores: Scores {ct: 0, t: 0},
            round_start: None,
            round_live: false,
            bomb_down: false,
            map: "".to_string(),
        }
    }
}

impl State {
    fn handle_message(&mut self, msg: &message::Message) {
        self.gsi.update();
        if let Ok(last_up) = tm_from_unix_timestamp(msg.provider.timestamp) {
            self.last_up = last_up;
        }
        if let Some(ref map) = msg.map {
            self.gamemode = map.clone().mode;
            self.map = map.clone().name;
            self.scores.ct = map.team_ct.score;
            self.scores.t = map.team_t.score;
            self.in_game = self.gamemode == message::Mode::competitive;

            if let Some(ref round) = msg.round {
                let live = message::round::Phase::live;
                self.round_live = round.phase == live;
                if let Some(ref prev) = msg.previously {
                    if let Some(ref prev_round) = prev.round {
                        if let Some(ref prev_phase) = prev_round.phase {
                            if *prev_phase != live && round.phase == live {
                                self.round_start = Some(self.last_up);
                            }
                        }
                        if let Some(_) = prev_round.bomb {
                            self.bomb_down = false;
                        }
                    }
                }
            } else {
                self.round_live = false;
            }
        } else {
            self.in_game = false;
        }
        if let Some(ref added) = msg.added {
            if let Some(ref a_round) = added.round {
                if let Some(ref a_r_bomb) = a_round.bomb {
                    if *a_r_bomb {
                        self.bomb_down = true;
                        self.round_start = Some(self.last_up);
                    }
                }
            }
        }
        let ref provider = msg.provider;
        let ref player = msg.player;
        if provider.steamid == player.steamid {
            if let Some(team) = player.team {
                self.team = Some(team);
            }
        }
    }
}

fn tm_from_unix_timestamp(timestamp: u32) -> Result<time::Tm, time::ParseError> {
    let timestamp_as_string = timestamp.to_string();
    time::strptime(&timestamp_as_string, "%s")
}

impl TakesUpdates for State {
    fn update(&mut self, reason: &UpdateReason) {
        match *reason {
            UpdateReason::Fetch => {
                self.gsi.update();
            },
            UpdateReason::Update => {
                self.gsi.invalidate();
            },
            UpdateReason::Data(ref message) => {
                self.handle_message(message);
            }
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let twenty_seconds = time::Duration::seconds(20);
        let twenty_seconds_ago = time::now() - twenty_seconds;
        let is_up = self.last_up > twenty_seconds_ago;
        d.insert("up".to_string(), is_up.to_json());
        d.insert("in_game".to_string(), self.in_game.to_json());
        if let Some(ref team) = self.team {
            d.insert("team".to_string(), team.to_json());
        }
        d.insert("gsi".to_string(), self.gsi.to_json());
        d.insert("gamemode".to_string(), self.gamemode.to_json());
        d.insert("scores".to_string(), self.scores.to_json());
        d.insert("round_live".to_string(), self.round_live.to_json());
        if let Some(ref round_start) = self.round_start {
            let mut max_round_duration = time::Duration::seconds(40);
            if !self.bomb_down {
                max_round_duration = time::Duration::minutes(1) + time::Duration::seconds(55) + max_round_duration;
            }
            let round_end = *round_start + max_round_duration;
            if let Ok(round_end) = time::strftime("%s", &round_end) {
                d.insert("round_end".to_string(), round_end.to_json());
            }
        }
        d.insert("map".to_string(), self.map.to_json());
        if let Ok(prefs) = Prefs::get() {
            d.insert("settings".to_string(), prefs.to_json());
        }
        Json::Object(d)
    }
}
