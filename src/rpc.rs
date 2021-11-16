const VERSION: &'static str = env!("CARGO_PKG_VERSION");

extern crate discord_rpc_client;
use discord_rpc_client::Client as DiscordRPC;

pub fn turn_on() {
    let mut drpc = DiscordRPC::new(909791454040301568);

    drpc.start();

        drpc.set_activity(|a| {
            a.state("Edditing valerriyya.rs").assets(|asset| {
                asset.large_image("valeriyya_pfp")
                    .large_text(format!("Valeriyya v{}", VERSION))
                    .small_image("rust_logo")
                    .small_text("coding away...")
            })
        });
        println!("Successfully enabled rich presence.");
        
}