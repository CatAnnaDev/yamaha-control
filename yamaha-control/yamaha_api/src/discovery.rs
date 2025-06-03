use crate::async_api::YamahaAmpAsync;
use crate::error::YamahaError;
use crate::YamahaAmpBlocking;
use futures::stream::{FuturesUnordered, StreamExt};
use std::net::Ipv4Addr;
use tokio::time::{timeout, Duration};

/// Configuration pour la découverte des amplificateurs sur le réseau
#[derive(Debug, Clone, Copy)]
pub struct DiscoveryConfig {
    /// Adresse du sous-réseau à scanner
    pub subnet: Ipv4Addr,
    /// Masque de sous-réseau (en notation CIDR)
    pub mask: u8,
    /// Délai d'attente pour chaque requête
    pub timeout: Duration,
    /// Nombre maximum de connexions simultanées
    pub max_concurrent: usize,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            subnet: Ipv4Addr::new(192, 168, 1, 0),
            mask: 24,
            timeout: Duration::from_millis(500),
            max_concurrent: 50,
        }
    }
}

/// Découvre de manière asynchrone les amplificateurs Yamaha sur le réseau
///
/// # Arguments
/// * `config` - Configuration optionnelle pour la découverte
///
/// # Returns
/// * `Result<Vec<YamahaAmpAsync>, YamahaError>` - Liste des amplificateurs trouvés
pub async fn discover_amplifiers(
    config: Option<DiscoveryConfig>,
) -> Result<Vec<YamahaAmpAsync>, YamahaError> {
    let config = config.unwrap_or_default();
    let ips = generate_ip_range(&config.subnet, config.mask);
    let mut found = Vec::new();

    let mut tasks = FuturesUnordered::new();
    let client = build_client(&config)?;

    for chunk in ips.chunks(config.max_concurrent) {
        for &ip in chunk {
            tasks.push(try_connect_with_client(ip, client.clone(), config.timeout));
        }

        while let Some(result) = tasks.next().await {
            if let Ok(Some(amp)) = result {
                found.push(amp);
            }
        }
    }

    Ok(found)
}

/// Crée un client HTTP asynchrone avec la configuration spécifiée
///
/// # Arguments
/// * `config` - Configuration pour le client
///
/// # Returns
/// * `Result<reqwest::Client, YamahaError>` - Client HTTP configuré
fn build_client(config: &DiscoveryConfig) -> Result<reqwest::Client, YamahaError> {
    reqwest::Client::builder()
        .timeout(config.timeout)
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Some(Duration::from_secs(30)))
        .build()
        .map_err(YamahaError::from)
}

/// Tente de se connecter à un amplificateur de manière asynchrone
///
/// # Arguments
/// * `ip` - Adresse IP à tester
/// * `client` - Client HTTP à utiliser
/// * `timeout_duration` - Délai d'attente maximum
///
/// # Returns
/// * `Result<Option<YamahaAmpAsync>, YamahaError>` - Amplificateur trouvé ou None
async fn try_connect_with_client(
    ip: Ipv4Addr,
    client: reqwest::Client,
    timeout_duration: Duration,
) -> Result<Option<YamahaAmpAsync>, YamahaError> {
    let url = format!(
        "http://{}/YamahaExtendedControl/v1/system/getDeviceInfo",
        ip
    );

    match timeout(timeout_duration, client.get(&url).send()).await {
        Ok(Ok(resp)) => {
            let json = resp.json::<serde_json::Value>().await?;
            Ok(Some(YamahaAmpAsync::from_discovery(ip, json)))
        }
        _ => Ok(None),
    }
}

/// Tente de se connecter à un amplificateur de manière synchrone
///
/// # Arguments
/// * `ip` - Adresse IP à tester
/// * `client` - Client HTTP synchrone à utiliser
///
/// # Returns
/// * `Result<Option<YamahaAmpBlocking>, YamahaError>` - Amplificateur trouvé ou None
fn try_connect_blocking_with_client(
    ip: Ipv4Addr,
    client: &reqwest::blocking::Client,
) -> Result<Option<YamahaAmpBlocking>, YamahaError> {
    let url = format!(
        "http://{}/YamahaExtendedControl/v1/system/getDeviceInfo",
        ip
    );

    match client.get(&url).send() {
        Ok(resp) => match resp.json::<serde_json::Value>() {
            Ok(json) => Ok(Some(YamahaAmpBlocking::from_discovery(ip, json))),
            Err(_) => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

/// Génère une liste d'adresses IP à partir d'un sous-réseau
///
/// # Arguments
/// * `subnet` - Adresse du sous-réseau
/// * `mask` - Masque de sous-réseau en notation CIDR
///
/// # Returns
/// Liste des adresses IP à scanner
fn generate_ip_range(subnet: &Ipv4Addr, mask: u8) -> Vec<Ipv4Addr> {
    let start = u32::from(*subnet) & (u32::MAX << (32 - mask));
    let end = start | (u32::MAX >> mask);

    let capacity = (end - start + 1) as usize;
    let mut ips = Vec::with_capacity(capacity);

    for ip in start..=end {
        if (ip & 0xFF) != 0 && (ip & 0xFF) != 255 {
            ips.push(Ipv4Addr::from(ip));
        }
    }

    ips
}

/// Découvre de manière synchrone les amplificateurs Yamaha sur le réseau
///
/// # Arguments
/// * `config` - Configuration optionnelle pour la découverte
///
/// # Returns
/// * `Result<Vec<YamahaAmpBlocking>, YamahaError>` - Liste des amplificateurs trouvés
pub fn discover_amplifiers_blocking(
    config: Option<DiscoveryConfig>,
) -> Result<Vec<YamahaAmpBlocking>, YamahaError> {
    use rayon::prelude::*;

    let config = config.unwrap_or_default();
    let ips = generate_ip_range(&config.subnet, config.mask);
    let client = build_blocking_client(&config)?;

    Ok(ips
        .into_par_iter()
        .filter_map(|ip| try_connect_blocking_with_client(ip, &client).ok().flatten())
        .collect())
}

/// Crée un client HTTP synchrone avec la configuration spécifiée
///
/// # Arguments
/// * `config` - Configuration pour le client
///
/// # Returns
/// * `Result<reqwest::blocking::Client, YamahaError>` - Client HTTP synchrone configuré
fn build_blocking_client(
    config: &DiscoveryConfig,
) -> Result<reqwest::blocking::Client, YamahaError> {
    reqwest::blocking::Client::builder()
        .timeout(config.timeout)
        .build()
        .map_err(YamahaError::from)
}

/// Se connecte directement à un amplificateur de manière asynchrone
///
/// # Arguments
/// * `ip` - Adresse IP de l'amplificateur
///
/// # Returns
/// * `Result<Option<YamahaAmpAsync>, YamahaError>` - Amplificateur trouvé ou None
pub async fn connect_direct(ip: Ipv4Addr) -> Result<Option<YamahaAmpAsync>, YamahaError> {
    let cfg = DiscoveryConfig::default();
    let config = build_client(&cfg)?;
    try_connect_with_client(ip, config, cfg.timeout).await
}

/// Se connecte directement à un amplificateur de manière synchrone
///
/// # Arguments
/// * `ip` - Adresse IP de l'amplificateur
///
/// # Returns
/// * `Result<Option<YamahaAmpBlocking>, YamahaError>` - Amplificateur trouvé ou None
pub fn connect_direct_blocking(ip: Ipv4Addr) -> Result<Option<YamahaAmpBlocking>, YamahaError> {
    let cfg = DiscoveryConfig::default();
    let client = build_blocking_client(&cfg)?;
    try_connect_blocking_with_client(ip, &client).map_err(YamahaError::from)
}
