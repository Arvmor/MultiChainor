use MultiChainor::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize environment variables
    init_env_vars();
    let chains = [Url::from_str("wss://eth.merkle.io")?];

    for chain in chains {
        // Create a provider for each chain
        let provider = ProviderBuilder::new().on_ws(WsConnect::new(chain)).await?;

        tokio::spawn(async move {
            // Subscribe to logs for each provider
            let mut stream = provider
                .subscribe_logs(&build_locked_liquidity_filter())
                .await
                .expect("Failed to subscribe to logs");

            // Listen to logs
            loop {
                if let Ok(log) = stream.recv().await {
                    // Decode the log
                    let decoded = decode_it(&log);
                    if decoded.is_none() {
                        continue;
                    }

                    // Send a message to Discord
                    send_discord_message(decoded).ok();
                }
            }
        });
    }

    // Wait for a Ctrl-C signal
    Ok(tokio::signal::ctrl_c().await?)
}
