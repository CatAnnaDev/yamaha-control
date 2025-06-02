use std::net::Ipv4Addr;
use yamaha_api::{SoundProgram, YamahaAmp, YamahaAmpBlocking, Zone};

#[tokio::main]
async fn main() {

    // Mode async
    let amps = YamahaAmp::discover().await;
    for (index, amp) in amps.iter().enumerate() {
        println!("Async found {}. {} -> ({})", index + 1, amp.model, amp.ip,);
    }

    let amp = YamahaAmp::connect(Ipv4Addr::new(192, 168, 1, 126)).await;
    if let Some(amp) = amp {
        println!("Directly Connected async to {}", amp.model);

        if let Ok(e) = amp.get_zone_status(Zone::Main).await {
            println!("Main: {}", e);
        }
        
        if let Err(e) = amp.set_sound_program(SoundProgram::Straight).await {
            eprintln!("Error: {:?}", e);      
        }
    }

    println!();

    // Mode sync
    tokio::task::spawn_blocking(|| {
        let amps = YamahaAmpBlocking::discover();
        for (index, amp) in amps.iter().enumerate() {
            println!("Sync found {}. {} -> ({})", index + 1, amp.model, amp.ip);
        }

        let amp = YamahaAmpBlocking::connect(Ipv4Addr::new(192, 168, 1, 126));
        if let Some(amp) = amp {
            println!("Directly Connected sync to {}", amp.model);

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
