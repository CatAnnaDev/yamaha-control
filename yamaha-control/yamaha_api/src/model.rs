use std::fmt::{Display, Formatter};
use std::net::IpAddr;

use serde_json::Value;

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
            YamahaErrorCode::StreamingNeedPlaylistDestination => "113 Playlist Destination Required".into(),
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

#[derive(Debug, Clone)]
pub struct YamahaAmp {
    pub ip: IpAddr,
    pub model: String,
    pub device_id: String,
    pub api_version: String,
}

impl YamahaAmp {
    pub fn from_discovery(ip: IpAddr, json: serde_json::Value) -> Self {
        Self {
            ip,
            model: json.get("model_name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            device_id: json.get("device_id").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            api_version: json.get("api_version").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        }
    }

    pub fn endpoint(&self, path: &str) -> String {
        format!("http://{}/YamahaExtendedControl/v1/{}", self.ip, path)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Main,
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
