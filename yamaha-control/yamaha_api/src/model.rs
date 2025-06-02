use std::fmt;
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YamahaErrorCode {
    Ok,
    Initializing,
    InternalError,
    InvalidRequest,
    InvalidParameter,
    Guarded,
    Timeout,
    FirmwareUpdating,
    StreamingAccessError,
    StreamingOtherError,
    StreamingWrongUsername,
    StreamingWrongPassword,
    StreamingAccountExpired,
    StreamingAccountDisconnected,
    StreamingLimitReached,
    StreamingMaintenance,
    StreamingInvalidAccount,
    StreamingLicenseError,
    StreamingReadOnly,
    StreamingMaxStations,
    StreamingAccessDenied,
    StreamingNeedPlaylistDestination,
    StreamingNeedNewPlaylist,
    StreamingSimultaneousLogins,
    DistributionLinking,
    DistributionUnlinking,
    InvalidResponse,
    Unknown(i32),
}

impl YamahaErrorCode {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => Self::Ok,
            1 => Self::Initializing,
            2 => Self::InternalError,
            3 => Self::InvalidRequest,
            4 => Self::InvalidParameter,
            5 => Self::Guarded,
            6 => Self::Timeout,
            99 => Self::FirmwareUpdating,
            100 => Self::StreamingAccessError,
            101 => Self::StreamingOtherError,
            102 => Self::StreamingWrongUsername,
            103 => Self::StreamingWrongPassword,
            104 => Self::StreamingAccountExpired,
            105 => Self::StreamingAccountDisconnected,
            106 => Self::StreamingLimitReached,
            107 => Self::StreamingMaintenance,
            108 => Self::StreamingInvalidAccount,
            109 => Self::StreamingLicenseError,
            110 => Self::StreamingReadOnly,
            111 => Self::StreamingMaxStations,
            112 => Self::StreamingAccessDenied,
            113 => Self::StreamingNeedPlaylistDestination,
            114 => Self::StreamingNeedNewPlaylist,
            115 => Self::StreamingSimultaneousLogins,
            200 => Self::DistributionLinking,
            201 => Self::DistributionUnlinking,
            _ => Self::Unknown(code),
        }
    }

    pub fn message(&self) -> String {
        match self {
            YamahaErrorCode::Ok => "0 Successful request".into(),
            YamahaErrorCode::Initializing => "1 Initializing".into(),
            YamahaErrorCode::InternalError => "2 Internal Error".into(),
            YamahaErrorCode::InvalidRequest => "3 Invalid Request".into(),
            YamahaErrorCode::InvalidParameter => "4 Invalid Parameter".into(),
            YamahaErrorCode::Guarded => "5 Guarded".into(),
            YamahaErrorCode::Timeout => "6 Timeout".into(),
            YamahaErrorCode::FirmwareUpdating => "99 Firmware Updating".into(),
            YamahaErrorCode::Unknown(code) => format!("{code} Unknown Error Code"),
            YamahaErrorCode::InvalidResponse => "Invalid Response".into(),
            YamahaErrorCode::StreamingAccessError => "100 Streaming Access Error".into(),
            YamahaErrorCode::StreamingOtherError => "101 Streaming Other Error".into(),
            YamahaErrorCode::StreamingWrongUsername => "102 Wrong Username".into(),
            YamahaErrorCode::StreamingWrongPassword => "103 Wrong Password".into(),
            YamahaErrorCode::StreamingAccountExpired => "104 Account Expired".into(),
            YamahaErrorCode::StreamingAccountDisconnected => "105 Account Disconnected".into(),
            YamahaErrorCode::StreamingLimitReached => "106 Limit Reached".into(),
            YamahaErrorCode::StreamingMaintenance => "107 Server Maintenance".into(),
            YamahaErrorCode::StreamingInvalidAccount => "108 Invalid Account".into(),
            YamahaErrorCode::StreamingLicenseError => "109 License Error".into(),
            YamahaErrorCode::StreamingReadOnly => "110 Read Only Mode".into(),
            YamahaErrorCode::StreamingMaxStations => "111 Max Stations Reached".into(),
            YamahaErrorCode::StreamingAccessDenied => "112 Access Denied".into(),
            YamahaErrorCode::StreamingNeedPlaylistDestination => {
                "113 Playlist Destination Required".into()
            }
            YamahaErrorCode::StreamingNeedNewPlaylist => "114 New Playlist Required".into(),
            YamahaErrorCode::StreamingSimultaneousLogins => "115 Simultaneous Login Limit".into(),
            YamahaErrorCode::DistributionLinking => "200 Linking in progress".into(),
            YamahaErrorCode::DistributionUnlinking => "201 Unlinking in progress".into(),
        }
    }
}

pub fn parse_response_code(data: &str) -> YamahaErrorCode {
    let parsed: Result<Value, _> = serde_json::from_str(data);
    if let Ok(value) = parsed {
        if let Some(code) = value["response_code"].as_i64() {
            return YamahaErrorCode::from_code(code as i32);
        }
    }
    YamahaErrorCode::Unknown(-1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Main,
    /// Zone B is handles as "Zone2"
    Zone2,
    Zone3,
    Zone4,
}

impl Zone {
    pub fn as_str(&self) -> &'static str {
        match self {
            Zone::Main => "main",
            Zone::Zone2 => "zone2",
            Zone::Zone3 => "zone3",
            Zone::Zone4 => "zone4",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "main" => Some(Zone::Main),
            "zone2" => Some(Zone::Zone2),
            "zone3" => Some(Zone::Zone3),
            "zone4" => Some(Zone::Zone4),
            _ => None,
        }
    }
}

pub enum PowerState{
    On,
    Toggle,
    Standby,
}

impl PowerState {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerState::On => "on",
            PowerState::Toggle => "toggle",
            PowerState::Standby => "standby",
        }
    }
}

impl Display for Zone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Zone::Main => f.write_str("main"),
            Zone::Zone2 => f.write_str("zone1"),
            Zone::Zone3 => f.write_str("zone2"),
            Zone::Zone4 => f.write_str("zone3"),
        }
    }
}



pub enum Input {
    Cd,
    Tuner,
    MultiCh,
    Phono,
    Hdmi1,
    Hdmi2,
    Hdmi3,
    Hdmi4,
    Hdmi5,
    Hdmi6,
    Hdmi7,
    Hdmi8,
    Hdmi,
    Av1,
    Av2,
    Av3,
    Av4,
    Av5,
    Av6,
    Av7,
    VAux,
    Aux1,
    Aux2,
    Aux,
    Audio1,
    Audio2,
    Audio3,
    Audio4,
    AudioCd,
    Audio,
    Optical1,
    Optical2,
    Optical,
    Coaxial1,
    Coaxial2,
    Coaxial,
    Digital1,
    Digital2,
    Digital,
    Line1,
    Line2,
    Line3,
    LineCd,
    Analog,
    Tv,
    BdDvd,
    UsbDac,
    Usb,
    Bluetooth,
    Server,
    NetRadio,
    Rhapsody,
    Napster,
    Pandora,
    Siriusxm,
    Spotify,
    Juke,
    Airplay,
    Radiko,
    Qobuz,
    McLink,
    MainSync,
    None,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Input::Cd => "cd",
            Input::Tuner => "tuner",
            Input::MultiCh => "multi_ch",
            Input::Phono => "phono",
            Input::Hdmi1 => "hdmi1",
            Input::Hdmi2 => "hdmi2",
            Input::Hdmi3 => "hdmi3",
            Input::Hdmi4 => "hdmi4",
            Input::Hdmi5 => "hdmi5",
            Input::Hdmi6 => "hdmi6",
            Input::Hdmi7 => "hdmi7",
            Input::Hdmi8 => "hdmi8",
            Input::Hdmi => "hdmi",
            Input::Av1 => "av1",
            Input::Av2 => "av2",
            Input::Av3 => "av3",
            Input::Av4 => "av4",
            Input::Av5 => "av5",
            Input::Av6 => "av6",
            Input::Av7 => "av7",
            Input::VAux => "v_aux",
            Input::Aux1 => "aux1",
            Input::Aux2 => "aux2",
            Input::Aux => "aux",
            Input::Audio1 => "audio1",
            Input::Audio2 => "audio2",
            Input::Audio3 => "audio3",
            Input::Audio4 => "audio4",
            Input::AudioCd => "audio_cd",
            Input::Audio => "audio",
            Input::Optical1 => "optical1",
            Input::Optical2 => "optical2",
            Input::Optical => "optical",
            Input::Coaxial1 => "coaxial1",
            Input::Coaxial2 => "coaxial2",
            Input::Coaxial => "coaxial",
            Input::Digital1 => "digital1",
            Input::Digital2 => "digital2",
            Input::Digital => "digital",
            Input::Line1 => "line1",
            Input::Line2 => "line2",
            Input::Line3 => "line3",
            Input::LineCd => "line_cd",
            Input::Analog => "analog",
            Input::Tv => "tv",
            Input::BdDvd => "bd_dvd",
            Input::UsbDac => "usb_dac",
            Input::Usb => "usb",
            Input::Bluetooth => "bluetooth",
            Input::Server => "server",
            Input::NetRadio => "net_radio",
            Input::Rhapsody => "rhapsody",
            Input::Napster => "napster",
            Input::Pandora => "pandora",
            Input::Siriusxm => "siriusxm",
            Input::Spotify => "spotify",
            Input::Juke => "juke",
            Input::Airplay => "airplay",
            Input::Radiko => "radiko",
            Input::Qobuz => "qobuz",
            Input::McLink => "mc_link",
            Input::MainSync => "main_sync",
            Input::None => "none",
        };
        write!(f, "{}", s)
    }
}


pub enum SoundProgram {
    MunichA,
    MunichB,
    Munich,
    Frankfurt,
    Stuttgart,
    Vienna,
    Amsterdam,
    UsaA,
    UsaB,
    Tokyo,
    Freiburg,
    Royaumont,
    Chamber,
    Concert,
    VillageGate,
    VillageVanguard,
    WarehouseLoft,
    CellarClub,
    JazzClub,
    RoxyTheatre,
    BottomLine,
    Arena,
    Sports,
    ActionGame,
    RoleplayingGame,
    Game,
    MusicVideo,
    Music,
    RecitalOpera,
    Pavilion,
    Disco,
    Standard,
    Spectacle,
    SciFi,
    Adventure,
    Drama,
    TalkShow,
    TvProgram,
    MonoMovie,
    Movie,
    Enhanced,
    Ch2Stereo,
    Ch5Stereo,
    Ch7Stereo,
    Ch9Stereo,
    Ch11Stereo,
    Stereo,
    SurrDecoder,
    MySurround,
    Target,
    Straight,
    Off,
}

impl fmt::Display for SoundProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SoundProgram::MunichA => "munich_a",
            SoundProgram::MunichB => "munich_b",
            SoundProgram::Munich => "munich",
            SoundProgram::Frankfurt => "frankfurt",
            SoundProgram::Stuttgart => "stuttgart",
            SoundProgram::Vienna => "vienna",
            SoundProgram::Amsterdam => "amsterdam",
            SoundProgram::UsaA => "usa_a",
            SoundProgram::UsaB => "usa_b",
            SoundProgram::Tokyo => "tokyo",
            SoundProgram::Freiburg => "freiburg",
            SoundProgram::Royaumont => "royaumont",
            SoundProgram::Chamber => "chamber",
            SoundProgram::Concert => "concert",
            SoundProgram::VillageGate => "village_gate",
            SoundProgram::VillageVanguard => "village_vanguard",
            SoundProgram::WarehouseLoft => "warehouse_loft",
            SoundProgram::CellarClub => "cellar_club",
            SoundProgram::JazzClub => "jazz_club",
            SoundProgram::RoxyTheatre => "roxy_theatre",
            SoundProgram::BottomLine => "bottom_line",
            SoundProgram::Arena => "arena",
            SoundProgram::Sports => "sports",
            SoundProgram::ActionGame => "action_game",
            SoundProgram::RoleplayingGame => "roleplaying_game",
            SoundProgram::Game => "game",
            SoundProgram::MusicVideo => "music_video",
            SoundProgram::Music => "music",
            SoundProgram::RecitalOpera => "recital_opera",
            SoundProgram::Pavilion => "pavilion",
            SoundProgram::Disco => "disco",
            SoundProgram::Standard => "standard",
            SoundProgram::Spectacle => "spectacle",
            SoundProgram::SciFi => "sci-fi",
            SoundProgram::Adventure => "adventure",
            SoundProgram::Drama => "drama",
            SoundProgram::TalkShow => "talk_show",
            SoundProgram::TvProgram => "tv_program",
            SoundProgram::MonoMovie => "mono_movie",
            SoundProgram::Movie => "movie",
            SoundProgram::Enhanced => "enhanced",
            SoundProgram::Ch2Stereo => "2ch_stereo",
            SoundProgram::Ch5Stereo => "5ch_stereo",
            SoundProgram::Ch7Stereo => "7ch_stereo",
            SoundProgram::Ch9Stereo => "9ch_stereo",
            SoundProgram::Ch11Stereo => "11ch_stereo",
            SoundProgram::Stereo => "stereo",
            SoundProgram::SurrDecoder => "surr_decoder",
            SoundProgram::MySurround => "my_surround",
            SoundProgram::Target => "target",
            SoundProgram::Straight => "straight",
            SoundProgram::Off => "off",
        };
        write!(f, "{}", s)
    }
}