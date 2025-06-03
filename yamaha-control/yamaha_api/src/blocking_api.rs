use crate::common_api::{ApiResponse, DeviceInfo, YamahaAmpBase, YamahaApi};
use crate::error::{YamahaError, YamahaErrorCode};
use crate::model::Zone;
use crate::{Input, PowerState, SoundProgram};
use reqwest::blocking::Client as BlockingClient;
use std::net::Ipv4Addr;
use std::ops::Deref;

impl Deref for YamahaAmpBlocking {
    type Target = YamahaAmpBase<BlockingClient>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Structure représentant un amplificateur Yamaha avec une interface synchrone
#[derive(Debug)]
pub struct YamahaAmpBlocking(YamahaAmpBase<BlockingClient>);

impl YamahaApi for YamahaAmpBlocking {}

impl YamahaAmpBlocking {
    /// Crée une nouvelle instance à partir d'une découverte réseau
    pub(crate) fn from_discovery(ip: Ipv4Addr, json: serde_json::Value) -> Self {
        Self(YamahaAmpBase {
            ip,
            client: BlockingClient::new(),
            info: serde_json::from_value::<DeviceInfo>(json).unwrap_or_default(),
        })
    }

    /// Effectue une requête HTTP GET synchrone vers l'amplificateur
    ///
    /// # Arguments
    /// * `url` - L'URL complète de la requête
    ///
    /// # Returns
    /// * `Result<T, YamahaError>` - Le résultat désérialisé ou une erreur
    fn request<T: for<'de> serde::Deserialize<'de>>(&self, url: String) -> Result<T, YamahaError> {
        let response = self
            .0
            .client
            .get(url)
            .send()
            .map_err(YamahaError::Http)?
            .json::<ApiResponse<T>>()
            .map_err(YamahaError::Http)?;

        match (response.response_code, response.data) {
            (0, Some(data)) => Ok(data),
            (code, _) => Err(YamahaError::YamahaErrorCode(YamahaErrorCode::from_code(
                code,
            ))),
        }
    }

    /// Récupère le statut d'une zone spécifique
    ///
    /// # Arguments
    /// * `zone` - La zone pour laquelle récupérer le statut
    pub fn get_zone_status(&self, zone: Zone) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(&self.0.ip, &format!("{zone}/getStatus"));
        Self::request(self, url)
    }

    /// Récupère la liste des programmes sonores disponibles pour une zone
    pub fn get_sound_program_list(&self, zone: Zone) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(&self.0.ip, &format!("{zone}/getSoundProgramList"));
        Self::request(self, url)
    }

    /// Récupère le statut de la zone principale
    pub fn get_main_status(&self) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(&self.0.ip, "main/getStatus");
        Self::request(self, url)
    }

    /// Récupère les informations sur le signal audio
    pub fn get_signal_info(&self) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(&self.0.ip, "main/getSignalInfo");
        Self::request(self, url)
    }

    /// Définit le volume principal
    ///
    /// # Arguments
    /// * `volume` - Niveau de volume (généralement entre -80 et 16)
    pub fn set_volume(&self, volume: i32) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(&self.0.ip, &format!("main/setVolume?volume={}", volume));
        Self::request(self, url)
    }

    /// Définit le programme sonore
    pub fn set_sound_program(
        &self,
        program: SoundProgram,
    ) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!("main/setSoundProgram?program={}", program),
        );
        Self::request(self, url)
    }

    /// Contrôle l'alimentation de l'amplificateur
    pub fn set_power(&self, power_state: PowerState) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!("main/setPower?power={}", power_state.as_str()),
        );
        Self::request(self, url)
    }

    /// Active ou désactive le mode muet
    pub fn set_mute(&self, mute: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!("main/setMute?enable={}", mute.to_string().to_lowercase()),
        );
        Self::request(self, url)
    }

    /// Change la source d'entrée
    pub fn set_input(&self, input: Input) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(&self.0.ip, &format!("main/setInput?input={}", input));
        Self::request(self, url)
    }

    /// Active ou désactive le mode Direct
    pub fn set_direct(&self, direct: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!(
                "main/setDirect?enable={}",
                direct.to_string().to_lowercase()
            ),
        );
        Self::request(self, url)
    }

    /// Active ou désactive le mode Pure Direct
    pub fn set_pure_direct(&self, direct: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!(
                "main/setPureDirect?enable={}",
                direct.to_string().to_lowercase()
            ),
        );
        Self::request(self, url)
    }

    /// Active ou désactive l'amélioration du son
    pub fn set_enhancer(&self, enhance: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!(
                "main/setEnhancer?enable={}",
                enhance.to_string().to_lowercase()
            ),
        );
        Self::request(self, url)
    }

    /// Règle le niveau des dialogues
    pub fn set_dialogue_level(&self, level: i32) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!("main/setDialogueLevel?value={}", level),
        );
        Self::request(self, url)
    }

    /// Règle le volume du caisson de basse
    pub fn set_subwoofer_volume(&self, volume: i32) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!("main/setSubwooferVolume?volume={}", volume),
        );
        Self::request(self, url)
    }

    /// Active ou désactive l'extension des basses
    pub fn set_bass_extension(&self, extension: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!(
                "main/setBassExtension?enable={}",
                extension.to_string().to_lowercase()
            ),
        );
        Self::request(self, url)
    }

    /// Active ou désactive le mode Extra Bass
    pub fn set_extra_bass(&self, extra_bass: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!(
                "main/setExtraBass?enable={}",
                extra_bass.to_string().to_lowercase()
            ),
        );
        Self::request(self, url)
    }

    /// Active ou désactive le DRC adaptatif
    pub fn set_adaptative_drc(&self, drc: bool) -> Result<serde_json::Value, YamahaError> {
        let url = self.endpoint(
            &self.0.ip,
            &format!(
                "main/setAdaptativeDrc?enable={}",
                drc.to_string().to_lowercase()
            ),
        );
        Self::request(self, url)
    }
}
