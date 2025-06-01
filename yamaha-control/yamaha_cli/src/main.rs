use yamaha_api::{discover_amplifiers, Zone};

#[tokio::main]
async fn main() {
    println!("Recherche des amplis Yamaha...");
    let amplis = discover_amplifiers().await;

    if amplis.is_empty() {
        println!("Aucun ampli détecté.");
    } else {
        for (i, amp) in amplis.iter().enumerate() {
            println!("{}: {} ({})", i + 1, amp.model, amp.ip);
        }

        println!("\nStatut du premier ampli :");
        if let Err(status) = amplis[0].get_main_status().await {
            println!("Main: {}", status.message());
        }

        if let Err(status) = amplis[0].get_zone_status(Zone::Zone2).await {
            println!("Zone2: {}", status.message());
        }
    }
}
