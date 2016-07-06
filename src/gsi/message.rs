use rustc_serialize::json::{Json, ToJson};

pub enum UpdateReason {
    Fetch,
    Update,
    Data(Message),
}

pub trait TakesUpdates : Send {
    fn update(&mut self, data: &UpdateReason);
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Provider {
    pub steamid: String,
    pub timestamp: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Activity {
    menu,
    playing,
    textinput,
}

impl ToJson for Activity {
    fn to_json(&self) -> Json {
        Json::String((match *self {
            Activity::menu => "menu",
            Activity::playing => "playing",
            Activity::textinput => "textinput",
        }).to_string())
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Team {
    CT,
    T,
}

impl ToJson for Team {
    fn to_json(&self) -> Json {
        Json::String((match *self {
            Team::CT => "CT",
            Team::T => "T",
        }).to_string())
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Player {
    pub steamid: String,
    pub team: Option<Team>,
    pub activity: Option<Activity>,
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Mode {
    casual,
    competitive,
    gungametrbomb,
    gungameprogressive,
    deathmatch,
    custom,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::custom
    }
}

impl ToJson for Mode {
    fn to_json(&self) -> Json {
        use self::Mode::*;
        Json::String((match *self {
            casual => "Casual",
            competitive => "Competitive",
            gungametrbomb => "Demolition",
            gungameprogressive => "Arms Race",
            deathmatch => "Deathmatch",
            custom => "Custom",
        }).to_string())
    }
}

pub mod round {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Phase {
        over,
        live,
        freezetime,
    }

    impl Default for Phase {
        fn default() -> Phase {
            Phase::over
        }
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Bomb {
        planted,
        exploded,
        defused,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone, Copy)]
    pub struct Round {
        pub phase: Phase,
        pub bomb: Option<Bomb>,
        pub win_team: Option<super::Team>,
    }
}

mod map {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Phase {
        live,
        gameover,
        warmup,
        intermission,
    }

    impl Default for Phase {
        fn default() -> Phase {
            Phase::gameover
        }
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone, Copy)]
    pub struct Team {
        pub score: u8,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Map {
        pub mode: super::Mode,
        pub name: String,
        pub phase: Phase,
        pub team_ct: Team,
        pub team_t: Team,
    }
}

mod previous {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Provider {
        pub steamid: Option<String>,
        pub timestamp: Option<u32>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[allow(non_camel_case_types)]
    pub enum Activity {
        menu,
        playing,
        textinput,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Team {
        CT,
        T,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Player {
        pub steamid: Option<String>,
        pub team: Option<Team>,
        pub activity: Option<Activity>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Round {
        pub phase: Option<super::round::Phase>,
        pub bomb: Option<String>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Message {
        pub provider: Option<Provider>,
        pub player: Option<Player>,
        pub round: Option<Round>,
    }
}

mod added {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Round {
        pub bomb: Option<bool>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Message {
        pub round: Option<Round>,
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Message {
    pub provider: Provider,
    pub map: Option<map::Map>,
    pub round: Option<round::Round>,
    pub player: Player,
    pub previously: Option<previous::Message>,
    pub added: Option<added::Message>,
}
