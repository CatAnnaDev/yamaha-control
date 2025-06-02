use std::net::Ipv4Addr;
use yamaha_api::{YamahaAmp, YamahaAmpBlocking};

#[tokio::main]
async fn main() {
    
    // Mode async
    let amps = YamahaAmp::discover().await;
    for (index, amp) in amps.iter().enumerate() {
        println!("Async {}. {} -> ({})", index + 1, amp.model, amp.ip,);
    }

    let amp = YamahaAmp::connect(Ipv4Addr::new(192, 168, 1, 126)).await;
    if let Some(amp) = amp {
        println!("Connected async to {}", amp.model);
    }

    println!();

    // Mode sync
    tokio::task::spawn_blocking(|| {
        let amps = YamahaAmpBlocking::discover();
        for (index, amp) in amps.iter().enumerate() {
            println!("Sync {}. {} -> ({})", index + 1, amp.model, amp.ip);
        }

        let amp = YamahaAmpBlocking::connect(Ipv4Addr::new(192, 168, 1, 126));
        if let Some(amp) = amp {
            println!("Connected sync to {}", amp.model);
        }
    })
    .await
    .unwrap()
}
