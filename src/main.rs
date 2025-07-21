use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use std::time::Duration;
use tokio::time::sleep;

mod filter;
mod helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for chain in &helper::CHAINS.0 {
        // Create a provider for each chain
        let provider = ProviderBuilder::new()
            .connect_ws(WsConnect::new(chain.as_ref()))
            .await?;

        tokio::spawn(async move {
            // Subscribe to logs for each provider
            let mut stream = provider
                .subscribe_logs(&filter::build_locked_liquidity_filter())
                .await
                .expect("Failed to subscribe to logs");

            // Listen to logs
            loop {
                match stream.recv().await {
                    // Decode the log
                    Ok(log) => {
                        if let Some((pool, date)) = filter::decode_it(&log) {
                            let decoded = format!("Address: {pool}, Unlock Date: <t:{date}:R>");

                            // Send a message to Discord
                            helper::send_discord_message(decoded).ok();
                        }
                    }
                    // Resubscribe if an error occurs
                    Err(_) => stream = stream.resubscribe(),
                }
            }
        });

        sleep(Duration::from_secs(10)).await;
    }

    // Wait for a Ctrl-C signal
    Ok(tokio::signal::ctrl_c().await?)
}
