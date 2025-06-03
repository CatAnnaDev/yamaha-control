use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::{fmt, io};

#[derive(Debug)]
pub enum YamahaError {
    Http(ReqwestError),
    Json(SerdeError),
    Io(io::Error),
    YamahaErrorCode(YamahaErrorCode),
    InvalidZone,
    Other(String),
    Deserialization(String),
}

impl fmt::Display for YamahaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YamahaError::Http(e) => write!(f, "HTTP error: {}", e),
            YamahaError::Json(e) => write!(f, "JSON error: {}", e),
            YamahaError::Io(e) => write!(f, "IO error: {}", e),
            YamahaError::YamahaErrorCode(code) => {
                write!(f, "Yamaha returned error code: {}", code.message())
            }
            YamahaError::InvalidZone => write!(f, "Invalid zone"),
            YamahaError::Other(msg) => write!(f, "{}", msg),
            YamahaError::Deserialization(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for YamahaError {}

impl From<ReqwestError> for YamahaError {
    fn from(e: ReqwestError) -> Self {
        YamahaError::Http(e)
    }
}

impl From<SerdeError> for YamahaError {
    fn from(e: SerdeError) -> Self {
        YamahaError::Json(e)
    }
}

impl From<io::Error> for YamahaError {
    fn from(e: io::Error) -> Self {
        YamahaError::Io(e)
    }
}

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
