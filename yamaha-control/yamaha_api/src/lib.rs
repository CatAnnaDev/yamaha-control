mod async_api;
mod blocking_api;
mod common_api;
mod discovery;
mod error;
mod json_data;
mod model;

pub use {
    async_api::YamahaAmpAsync, blocking_api::YamahaAmpBlocking, discovery::*, error::YamahaError,
    json_data::*, model::*,
};

use std::net::Ipv4Addr;

impl YamahaAmpAsync {
    /// Découvre tous les amplificateurs Yamaha sur le réseau
    ///
    /// # Arguments
    /// * `config` - Configuration optionnelle pour la découverte
    ///
    /// # Returns
    /// * `Result<Vec<YamahaAmpAsync>, YamahaError>` - Liste des amplificateurs trouvés
    pub async fn discover(config: Option<DiscoveryConfig>) -> Result<Vec<Self>, YamahaError> {
        discover_amplifiers(config).await
    }

    /// Se connecte directement à un amplificateur à l'adresse IP spécifiée
    ///
    /// # Arguments
    /// * `ip` - Adresse IPv4 de l'amplificateur
    ///
    /// # Returns
    /// * `Result<Option<YamahaAmpAsync>, YamahaError>` - L'amplificateur s'il est trouvé
    pub async fn connect(ip: Ipv4Addr) -> Result<Option<Self>, YamahaError> {
        connect_direct(ip).await
    }
}

impl YamahaAmpBlocking {
    /// Découvre tous les amplificateurs Yamaha sur le réseau
    ///
    /// # Arguments
    /// * `config` - Configuration optionnelle pour la découverte
    ///
    /// # Returns
    /// * `Result<Vec<YamahaAmpBlocking>, YamahaError>` - Liste des amplificateurs trouvés
    pub fn discover(config: Option<DiscoveryConfig>) -> Result<Vec<Self>, YamahaError> {
        discover_amplifiers_blocking(config)
    }

    /// Se connecte directement à un amplificateur à l'adresse IP spécifiée
    ///
    /// # Arguments
    /// * `ip` - Adresse IPv4 de l'amplificateur
    ///
    /// # Returns
    /// * `Result<Option<YamahaAmpBlocking>, YamahaError>` - L'amplificateur s'il est trouvé
    pub fn connect(ip: Ipv4Addr) -> Result<Option<Self>, YamahaError> {
        connect_direct_blocking(ip)
    }
}
