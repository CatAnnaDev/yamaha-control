mod async_api;
mod blocking_api;
mod discovery;
mod model;
mod json_data;

use crate::async_api::YamahaAmpAsync;
pub use crate::blocking_api::YamahaAmpBlocking;
pub use discovery::*;
pub use model::*;
use std::net::Ipv4Addr;
pub use json_data::*;

pub struct YamahaAmp;

impl YamahaAmp {
    pub async fn discover() -> Vec<YamahaAmpAsync> {
        discover_amplifiers().await
    }

    pub async fn connect(ip: Ipv4Addr) -> Option<YamahaAmpAsync> {
        connect_direct(ip).await
    }
}

impl YamahaAmpBlocking {
    pub fn discover() -> Vec<YamahaAmpBlocking> {
        discover_amplifiers_blocking()
    }

    pub fn connect(ip: Ipv4Addr) -> Option<YamahaAmpBlocking> {
        connect_direct_blocking(ip)
    }
}
