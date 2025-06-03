use serde_derive::Deserialize;
use std::net::Ipv4Addr;

/// Chemin de base pour l'API Yamaha Extended Control
pub const API_BASE_PATH: &str = "YamahaExtendedControl/v1";

/// Informations de base sur un périphérique Yamaha
#[derive(Debug, Deserialize, Default)]
pub struct DeviceInfo {
    /// Nom du modèle de l'appareil
    #[serde(rename = "model_name")]
    pub model: String,
    /// Identifiant unique de l'appareil
    pub device_id: String,
    /// Version de l'API supportée par l'appareil
    pub api_version: f32,
}

/// Structure générique pour les réponses de l'API
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// Code de réponse (0 = succès)
    pub response_code: i32,
    /// Données de la réponse, aplaties dans la structure
    #[serde(flatten)]
    pub data: Option<T>,
}

/// Trait définissant les fonctionnalités communes de l'API Yamaha
pub trait YamahaApi {
    /// Construit l'URL de base pour un endpoint de l'API
    ///
    /// # Arguments
    /// * `ip` - Adresse IP de l'appareil
    /// * `path` - Chemin de l'endpoint
    ///
    /// # Returns
    /// L'URL complète pour l'endpoint
    fn endpoint(&self, ip: &Ipv4Addr, path: &str) -> String {
        format!("http://{}/{}/{}", ip, API_BASE_PATH, path)
    }

    /// Construit une URL complète avec paramètres pour un endpoint de l'API
    ///
    /// # Arguments
    /// * `ip` - Adresse IP de l'appareil
    /// * `path` - Chemin de l'endpoint
    /// * `params` - Liste des paramètres sous forme de tuples (clé, valeur)
    ///
    /// # Returns
    /// L'URL complète avec les paramètres
    fn build_url(&self, ip: &Ipv4Addr, path: &str, params: &[(&str, &str)]) -> String {
        let base = self.endpoint(ip, path);
        if params.is_empty() {
            return base;
        }
        let params = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{}", base, params)
    }

    /// Convertit une valeur booléenne en chaîne de caractères pour l'API
    ///
    /// # Arguments
    /// * `value` - Valeur booléenne à convertir
    ///
    /// # Returns
    /// "true" ou "false" en tant que &str
    #[inline]
    fn bool_to_string(value: bool) -> &'static str {
        if value { "true" } else { "false" }
    }
}

/// Structure de base pour les implémentations d'amplificateurs Yamaha
///
/// # Type Parameters
/// * `C` - Type du client HTTP (async ou blocking)
#[derive(Debug)]
pub struct YamahaAmpBase<C> {
    /// Adresse IP de l'amplificateur
    pub ip: Ipv4Addr,
    /// Client HTTP pour les requêtes
    pub client: C,
    /// Informations sur le périphérique
    pub info: DeviceInfo,
}
