const VERSION: &'static str = env!("CARGO_PKG_VERSION");

extern crate discord_rpc_client;
use discord_rpc_client::Client as DiscordRPC;
use std::{thread, time};

pub fn turn_on() {
    let mut drpc = DiscordRPC::new(909791454040301568);

    drpc.start();
    println!("Successfully enabled rich presence.");
    loop {
        drpc.set_activity(|a| {
            a.state("Edditing valerriyya.rs").assets(|asset| {
                asset.large_image("valeriyya_pfp")
                    .large_text(format!("Valeriyya v{}", VERSION))
                    .small_image("rust_logo")
                    .small_text("coding away...")
            })
        });

        thread::sleep(time::Duration::from_secs(10));
    }
}