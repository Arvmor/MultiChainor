use MultiChainor::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize environment variables
    init_env_vars();
    let chains = [
        Url::from_str("wss://eth.merkle.io")?,
        Url::from_str("wss://base.gateway.tenderly.co")?,
        Url::from_str("wss://arbitrum.gateway.tenderly.co")?,
        // Url::from_str("wss://avalanche.gateway.tenderly.co")?,
        // Url::from_str("wss://polygon.gateway.tenderly.co")?,
    ];

    for chain in chains {
        // Create a provider for each chain
        let mut message = format!("Live with {chain} ");
        let provider = ProviderBuilder::new().on_ws(WsConnect::new(chain)).await?;

        tokio::spawn(async move {
            // Subscribe to logs for each provider
            let mut stream = provider
                .subscribe_logs(&build_locked_liquidity_filter())
                .await
                .expect("Failed to subscribe to logs");

            message += &stream.recv().await.is_ok().to_string();
            send_discord_message(message).ok();

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

        sleep(Duration::from_secs(10)).await;
    }

    // Wait for a Ctrl-C signal
    Ok(tokio::signal::ctrl_c().await?)
}
