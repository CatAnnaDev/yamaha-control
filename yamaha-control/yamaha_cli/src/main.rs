use std::net::Ipv4Addr;
use std::time::Duration;
use yamaha_api::{
    DiscoveryConfig, GetStatus, SoundProgram, YamahaAmpAsync, YamahaAmpBlocking, Zone,
};

#[tokio::main]
async fn main() {
    // Optional config
    let cfg = DiscoveryConfig {
        subnet: Ipv4Addr::new(192, 168, 1, 0),
        mask: 24,
        timeout: Duration::from_millis(200),
        max_concurrent: 50,
    };

    // Mode async
    let amps = YamahaAmpAsync::discover(Some(cfg))
        .await
        .expect("Failed to discover amps async");
    for (index, amp) in amps.iter().enumerate() {
        println!(
            "Async found {}. {} -> ({})",
            index + 1,
            amp.info.model,
            amp.ip,
        );
    }

    let amp = YamahaAmpAsync::connect(Ipv4Addr::new(192, 168, 1, 126))
        .await
        .expect("Failed to connect async");
    if let Some(amp) = amp {
        println!("Directly Connected async to {}", amp.info.model);

        if let Ok(e) = amp.get_zone_status(Zone::Main).await {
            match serde_json::from_value::<GetStatus>(e) {
                Ok(e) => println!("Main actual volume: {} / {}", e.volume, e.max_volume),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }

        if let Err(e) = amp.set_sound_program(SoundProgram::Straight).await {
            eprintln!("Error: {:?}", e);
        }
    }

    println!();

    // Mode sync
    tokio::task::spawn_blocking(move || {
        let amps = YamahaAmpBlocking::discover(Some(cfg)).expect("Failed to discover amps sync");
        for (index, amp) in amps.iter().enumerate() {
            println!(
                "Sync found {}. {} -> ({})",
                index + 1,
                amp.info.model,
                amp.ip
            );
        }

        let amp = YamahaAmpBlocking::connect(Ipv4Addr::new(192, 168, 1, 126))
            .expect("Failed to connect sync");
        if let Some(amp) = amp {
            println!("Directly Connected sync to {}", amp.info.model);

            if let Ok(e) = amp.get_zone_status(Zone::Main) {
                println!("Main: {}", e);
            }

            if let Err(e) = amp.set_sound_program(SoundProgram::Straight) {
                eprintln!("Error: {:?}", e);
            }
        }
    })
    .await
    .unwrap()
}
