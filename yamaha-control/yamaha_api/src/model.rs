macro_rules! impl_string_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident $(= $str_value:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )*
        }

        impl $name {
            fn to_snake_case(s: &str) -> String {
                let mut result = String::with_capacity(s.len() + 4);
                for (i, ch) in s.chars().enumerate() {
                    if ch.is_uppercase() {
                        if i != 0 {
                            result.push('_');
                        }
                        for lower_ch in ch.to_lowercase() {
                            result.push(lower_ch);
                        }
                    } else {
                        result.push(ch);
                    }
                }
                result
            }

            pub fn as_str(&self) -> String {
                match self {
                    $(
                        Self::$variant => {
                            let raw = impl_string_enum!(@stringify_or_expr $variant $(, $str_value)?);
                            Self::to_snake_case(raw)
                        },
                    )*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                $(
                    if s == Self::to_snake_case(impl_string_enum!(@stringify_or_expr $variant $(, $str_value)?)) {
                        return Some(Self::$variant);
                    }
                )*
                None
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = self.as_str();
                f.write_str(&s)
            }
        }
    };

    (@stringify_or_expr $variant:ident, $str_value:expr) => {
        $str_value
    };

    (@stringify_or_expr $variant:ident) => {
        stringify!($variant)
    };
}

impl_string_enum! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Main,
    /// Zone B is handles as "Zone2"
    Zone2,
    Zone3,
    Zone4,
}
    }

impl_string_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    On,
    Toggle,
    Standby,
}
    }

impl_string_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    }

impl_string_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    }
