use serde_derive::Deserialize;
use serde_derive::Serialize;

// GetDeviceInfo

/// category_code
/// 0 : reserved
/// 1 : AV Receiver
/// 2 : Sound Bar
/// 3 : Stereo Receiver
/// 4 : Subwoofer
/// 5 : Mini System
/// 6 : Desktop Audio 1
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceInfo {
    #[serde(rename = "analytics_info")]
    pub analytics_info: AnalyticsInfo,
    #[serde(rename = "api_version")]
    pub api_version: f64,
    #[serde(rename = "category_code")]
    pub category_code: i64,
    pub destination: String,
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[serde(rename = "net_module_num")]
    pub net_module_num: i64,
    #[serde(rename = "netmodule_checksum")]
    pub netmodule_checksum: String,
    #[serde(rename = "netmodule_generation")]
    pub netmodule_generation: i64,
    #[serde(rename = "netmodule_version")]
    pub netmodule_version: String,
    #[serde(rename = "operation_mode")]
    pub operation_mode: String,
    #[serde(rename = "response_code")]
    pub response_code: i64,
    #[serde(rename = "serial_number")]
    pub serial_number: String,
    #[serde(rename = "system_id")]
    pub system_id: String,
    #[serde(rename = "system_version")]
    pub system_version: f64,
    #[serde(rename = "update_data_type")]
    pub update_data_type: i64,
    #[serde(rename = "update_error_code")]
    pub update_error_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsInfo {
    pub uuid: String,
}

// GetFeatures
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatures {
    pub ccs: Ccs,
    pub distribution: Distribution,
    pub netusb: Netusb,
    #[serde(rename = "response_code")]
    pub response_code: i64,
    pub system: System,
    pub tuner: Tuner,
    pub zone: Vec<GetFeaturesZone>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ccs {
    pub supported: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Distribution {
    #[serde(rename = "client_max")]
    pub client_max: i64,
    #[serde(rename = "compatible_client")]
    pub compatible_client: Vec<i64>,
    #[serde(rename = "mc_surround")]
    pub mc_surround: McSurround,
    #[serde(rename = "server_zone_list")]
    pub server_zone_list: Vec<String>,
    pub version: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McSurround {
    #[serde(rename = "func_list")]
    pub func_list: Vec<String>,
    #[serde(rename = "master_role")]
    pub master_role: MasterRole,
    #[serde(rename = "slave_role")]
    pub slave_role: SlaveRole,
    pub version: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasterRole {
    #[serde(rename = "stereo_pair")]
    pub stereo_pair: bool,
    #[serde(rename = "subwoofer_pair")]
    pub subwoofer_pair: bool,
    #[serde(rename = "surround_pair")]
    pub surround_pair: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlaveRole {
    #[serde(rename = "subwoofer_pair")]
    pub subwoofer_pair: bool,
    #[serde(rename = "surround_pair_l_or_r")]
    pub surround_pair_l_or_r: bool,
    #[serde(rename = "surround_pair_lr")]
    pub surround_pair_lr: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Netusb {
    #[serde(rename = "func_list")]
    pub func_list: Vec<String>,
    #[serde(rename = "mc_playlist")]
    pub mc_playlist: McPlaylist,
    #[serde(rename = "net_radio_type")]
    pub net_radio_type: String,
    #[serde(rename = "play_queue")]
    pub play_queue: PlayQueue,
    pub preset: Preset,
    pub qobuz: Qobuz,
    #[serde(rename = "recent_info")]
    pub recent_info: RecentInfo,
    pub tidal: Tidal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McPlaylist {
    pub num: i64,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayQueue {
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    pub num: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Qobuz {
    #[serde(rename = "login_type")]
    pub login_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentInfo {
    pub num: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tidal {
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub bluetooth: Bluetooth,
    #[serde(rename = "func_list")]
    pub func_list: Vec<String>,
    #[serde(rename = "input_list")]
    pub input_list: Vec<InputList>,
    #[serde(rename = "web_control_url")]
    pub web_control_url: String,
    #[serde(rename = "zone_num")]
    pub zone_num: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bluetooth {
    #[serde(rename = "tx_connectivity_type_max")]
    pub tx_connectivity_type_max: i64,
    #[serde(rename = "update_cancelable")]
    pub update_cancelable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputList {
    #[serde(rename = "account_enable")]
    pub account_enable: bool,
    #[serde(rename = "distribution_enable")]
    pub distribution_enable: bool,
    pub id: String,
    #[serde(rename = "play_info_type")]
    pub play_info_type: String,
    #[serde(rename = "rename_enable")]
    pub rename_enable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tuner {
    #[serde(rename = "func_list")]
    pub func_list: Vec<String>,
    pub preset: Preset2,
    #[serde(rename = "range_step")]
    pub range_step: Vec<RangeStep>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preset2 {
    pub num: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeStep {
    pub id: String,
    pub max: i64,
    pub min: i64,
    pub step: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeaturesZone {
    #[serde(rename = "actual_volume_mode_list")]
    pub actual_volume_mode_list: Vec<String>,
    #[serde(rename = "ccs_supported")]
    #[serde(default)]
    pub ccs_supported: Vec<String>,
    #[serde(rename = "cursor_list")]
    pub cursor_list: Option<Vec<String>>,
    #[serde(rename = "func_list")]
    pub func_list: Vec<String>,
    pub id: String,
    #[serde(rename = "input_list")]
    pub input_list: Vec<String>,
    #[serde(rename = "link_audio_delay_list")]
    pub link_audio_delay_list: Option<Vec<String>>,
    #[serde(rename = "link_control_list")]
    pub link_control_list: Option<Vec<String>>,
    #[serde(rename = "menu_list")]
    pub menu_list: Option<Vec<String>>,
    #[serde(rename = "range_step")]
    pub range_step: Vec<RangeStep2>,
    #[serde(rename = "scene_num")]
    pub scene_num: Option<i64>,
    #[serde(rename = "sound_program_list")]
    #[serde(default)]
    pub sound_program_list: Vec<String>,
    #[serde(rename = "surr_decoder_type_list")]
    pub surr_decoder_type_list: Option<Vec<String>>,
    #[serde(rename = "tone_control_mode_list")]
    #[serde(default)]
    pub tone_control_mode_list: Vec<String>,
    #[serde(rename = "zone_b")]
    pub zone_b: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeStep2 {
    pub id: String,
    pub max: f64,
    pub min: f64,
    pub step: f64,
}

// GetNetworkStatus
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkStatus {
    #[serde(rename = "airplay_pin")]
    pub airplay_pin: String,
    pub connection: String,
    #[serde(rename = "default_gateway")]
    pub default_gateway: String,
    pub dhcp: bool,
    #[serde(rename = "dns_server_1")]
    pub dns_server_1: String,
    #[serde(rename = "dns_server_2")]
    pub dns_server_2: String,
    #[serde(rename = "each_module_ip_list")]
    pub each_module_ip_list: Vec<String>,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    pub ipv6: Ipv6,
    #[serde(rename = "mac_address")]
    pub mac_address: MacAddress,
    #[serde(rename = "musiccast_network")]
    pub musiccast_network: MusiccastNetwork,
    #[serde(rename = "network_name")]
    pub network_name: String,
    #[serde(rename = "response_code")]
    pub response_code: i64,
    #[serde(rename = "subnet_mask")]
    pub subnet_mask: String,
    #[serde(rename = "wireless_lan")]
    pub wireless_lan: WirelessLan,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ipv6 {
    pub address: String,
    pub enable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacAddress {
    #[serde(rename = "wired_lan")]
    pub wired_lan: String,
    #[serde(rename = "wireless_direct")]
    pub wireless_direct: String,
    #[serde(rename = "wireless_lan")]
    pub wireless_lan: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusiccastNetwork {
    pub ch: i64,
    #[serde(rename = "child_num")]
    pub child_num: i64,
    #[serde(rename = "device_type")]
    pub device_type: String,
    pub dfs: Dfs,
    #[serde(rename = "initial_join_running")]
    pub initial_join_running: bool,
    pub ready: bool,
    #[serde(rename = "wlan1_ch")]
    pub wlan1_ch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dfs {
    pub option: bool,
    #[serde(rename = "radar_ch")]
    pub radar_ch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WirelessLan {
    pub ch: i64,
    pub enable: bool,
    pub key: String,
    pub ssid: String,
    pub strength: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

// GetBluetoothInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBluetoothInfo {
    #[serde(rename = "bluetooth_device")]
    pub bluetooth_device: BluetoothDevice,
    #[serde(rename = "bluetooth_standby")]
    pub bluetooth_standby: bool,
    #[serde(rename = "bluetooth_tx_connectivity_type")]
    pub bluetooth_tx_connectivity_type: i64,
    #[serde(rename = "bluetooth_tx_setting")]
    pub bluetooth_tx_setting: bool,
    #[serde(rename = "response_code")]
    pub response_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothDevice {
    pub address: String,
    pub connected: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

// GetBluetoothDeviceList
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBluetoothDeviceList {
    #[serde(rename = "response_code")]
    pub response_code: i64,
    pub updating: bool,
    #[serde(rename = "device_list")]
    pub device_list: Vec<DeviceList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceList {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub address: String,
}

// GetFuncStatus
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFuncStatus {
    #[serde(rename = "hdmi_out_1")]
    pub hdmi_out_1: bool,
    #[serde(rename = "hdmi_standby_through")]
    pub hdmi_standby_through: String,
    pub headphone: bool,
    #[serde(rename = "response_code")]
    pub response_code: i64,
    #[serde(rename = "zone_b_volume_sync")]
    pub zone_b_volume_sync: bool,
}

// getNameText without zone_id
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNameTextWithoutID {
    #[serde(rename = "input_list")]
    pub input_list: Vec<InputList>,
    #[serde(rename = "response_code")]
    pub response_code: i64,
    #[serde(rename = "sound_program_list")]
    pub sound_program_list: Vec<SoundProgramList>,
    #[serde(rename = "zone_list")]
    pub zone_list: Vec<ZoneList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameTextInputList {
    pub id: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundProgramList {
    pub id: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneList {
    pub id: String,
    pub text: String,
}

// GetNameText?id=main with id
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNameTextWithID {
    pub id: String,
    #[serde(rename = "response_code")]
    pub response_code: i64,
    pub text: String,
}

// GetStatus
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatus {
    #[serde(rename = "actual_volume")]
    pub actual_volume: ActualVolume,
    #[serde(rename = "adaptive_drc")]
    pub adaptive_drc: bool,
    #[serde(rename = "contents_display")]
    pub contents_display: bool,
    #[serde(rename = "dialogue_level")]
    pub dialogue_level: i64,
    #[serde(rename = "disable_flags")]
    pub disable_flags: i64,
    #[serde(rename = "distribution_enable")]
    pub distribution_enable: bool,
    pub enhancer: bool,
    #[serde(rename = "extra_bass")]
    pub extra_bass: bool,
    pub input: String,
    #[serde(rename = "input_text")]
    pub input_text: String,
    #[serde(rename = "link_audio_delay")]
    pub link_audio_delay: String,
    #[serde(rename = "link_control")]
    pub link_control: String,
    #[serde(rename = "max_volume")]
    pub max_volume: i64,
    pub mute: bool,
    pub power: String,
    #[serde(rename = "pure_direct")]
    pub pure_direct: bool,
    pub sleep: i64,
    #[serde(rename = "sound_program")]
    pub sound_program: String,
    #[serde(rename = "subwoofer_volume")]
    pub subwoofer_volume: i64,
    #[serde(rename = "surr_decoder_type")]
    pub surr_decoder_type: String,
    #[serde(rename = "tone_control")]
    pub tone_control: ToneControl,
    pub volume: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActualVolume {
    pub mode: String,
    pub unit: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToneControl {
    pub bass: i64,
    pub mode: String,
    pub treble: i64,
}

// GetSoundProgramList
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSoundProgramList {
    #[serde(rename = "response_code")]
    pub response_code: i64,
    #[serde(rename = "sound_program_list")]
    pub sound_program_list: Vec<String>,
}

// GetSignalInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignalInfo {
    pub audio: Audio,
    #[serde(rename = "response_code")]
    pub response_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub bit: String,
    pub bitrate: i64,
    pub error: i64,
    pub format: String,
    pub fs: String,
}
