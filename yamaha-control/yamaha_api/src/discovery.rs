use crate::YamahaAmpBlocking;
use crate::async_api::YamahaAmpAsync;
use futures::stream::{FuturesUnordered, StreamExt};
use std::net::Ipv4Addr;
use tokio::time::{Duration, timeout};

pub async fn discover_amplifiers() -> Vec<YamahaAmpAsync> {
    let mut tasks = FuturesUnordered::new();

    for i in 1..=254 {
        let ip = Ipv4Addr::new(192, 168, 1, i);
        tasks.push(async move {
            match try_connect(ip).await {
                Some(amp) => Some(amp),
                None => None,
            }
        });
    }

    let mut found = Vec::new();
    while let Some(result) = tasks.next().await {
        if let Some(amp) = result {
            found.push(amp);
        }
    }

    found
}

async fn try_connect(ip: Ipv4Addr) -> Option<YamahaAmpAsync> {
    let url = format!(
        "http://{}/YamahaExtendedControl/v1/system/getDeviceInfo",
        ip
    );
    let client = reqwest::Client::new();
    if let Ok(Ok(resp)) = timeout(Duration::from_millis(500), client.get(url).send()).await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            return Some(YamahaAmpAsync::from_discovery(ip, json));
        }
    }
    None
}

pub async fn connect_direct(ip: Ipv4Addr) -> Option<YamahaAmpAsync> {
    try_connect(ip).await
}

fn try_connect_blocking(ip: Ipv4Addr) -> Option<YamahaAmpBlocking> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(400))
        .build()
        .ok()?;

    let url = format!(
        "http://{}/YamahaExtendedControl/v1/system/getDeviceInfo",
        ip
    );
    if let Ok(resp) = client.get(&url).send() {
        if let Ok(json) = resp.json::<serde_json::Value>() {
            return Some(YamahaAmpBlocking::from_discovery(ip, json));
        }
    }
    None
}

pub fn discover_amplifiers_blocking() -> Vec<YamahaAmpBlocking> {
    use rayon::prelude::*;

    (1..=254)
        .into_par_iter()
        .filter_map(|i| {
            let ip = Ipv4Addr::new(192, 168, 1, i);
            try_connect_blocking(ip)
        })
        .collect()
}

pub fn connect_direct_blocking(ip: Ipv4Addr) -> Option<YamahaAmpBlocking> {
    try_connect_blocking(ip)
}
