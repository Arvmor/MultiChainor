use MultiChainor::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize environment variables
    init_env_vars();

    for chain in &CHAINS.0 {
        // Create a provider for each chain
        let provider = ProviderBuilder::new()
            .on_ws(WsConnect::new(chain.as_ref()))
            .await?;

        tokio::spawn(async move {
            // Subscribe to logs for each provider
            let mut stream = provider
                .subscribe_logs(&build_locked_liquidity_filter())
                .await
                .expect("Failed to subscribe to logs");

            // Listen to logs
            loop {
                match stream.recv().await {
                    // Decode the log
                    Ok(log) => {
                        let decoded = match decode_it(&log) {
                            Some((pool, date)) => {
                                format!("Address: {pool}, Unlock Date: <t:{date}:R>")
                            }
                            None => continue,
                        };

                        // Send a message to Discord
                        send_discord_message(decoded).ok();
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
