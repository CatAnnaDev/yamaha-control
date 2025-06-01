use std::net::{IpAddr, Ipv4Addr};
use tokio::time::{timeout, Duration};
use futures::stream::{FuturesUnordered, StreamExt};
use crate::YamahaAmp;

pub async fn discover_amplifiers() -> Vec<YamahaAmp> {
    let mut tasks = FuturesUnordered::new();

    for i in 1..=254 {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, i));
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

async fn try_connect(ip: IpAddr) -> Option<YamahaAmp> {
    let url = format!("http://{}/YamahaExtendedControl/v1/system/getDeviceInfo", ip);
    let client = reqwest::Client::new();
    if let Ok(Ok(resp)) = timeout(Duration::from_millis(500), client.get(url).send()).await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            return Some(YamahaAmp::from_discovery(ip, json));
        }
    }
    None
}

pub async fn connect_direct(ip: IpAddr) -> Option<YamahaAmp> {
    let url = format!("http://{}/YamahaExtendedControl/v1/system/getDeviceInfo", ip);
    let client = reqwest::Client::new();
    if let Ok(resp) = client.get(url).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            return Some(YamahaAmp::from_discovery(ip, json));
        }
    }
    None
}