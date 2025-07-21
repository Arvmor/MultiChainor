use alloy::transports::http::reqwest::Url;
use serde::Serialize;
use serde_json::json;
use std::{error::Error, str::FromStr, sync::LazyLock};
use ureq::Agent;

// Ureq Agent
pub static UREQ_AGENT: LazyLock<Agent> = LazyLock::new(ureq::agent);
pub static DISCORD_WEBHOOK: LazyLock<Url> = LazyLock::new(|| {
    let url = std::env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK not found");
    Url::from_str(&url).expect("DISCORD_WEBHOOK is not a valid URL")
});
pub static CHAINS: LazyLock<Chains> = LazyLock::new(|| {
    let chains = std::env::var("CHAINS").expect("CHAINS not found");
    Chains::from_str(&chains).expect("CHAINS is not a valid URLs")
});

pub fn send_discord_message(message: impl Serialize) -> Result<(), Box<dyn Error>> {
    // Struct a message to Discord
    let value = json!({
        "content": "<@&1224787358789140520>",
        "embeds": [{
            "color": 5814783,
            "fields": [{
                "name": "-- Alert --",
                "value": message
            }]
            }
        ],
        "username": "MetaPro",
        "avatar_url": "https://metapro.app/img/icon-128.png"
    });

    // Send a message to Discord
    UREQ_AGENT
        .clone()
        .post(DISCORD_WEBHOOK.as_ref())
        .send_json(&value)?;

    Ok(())
}

/// Chains
pub struct Chains(pub Vec<Url>);
impl FromStr for Chains {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let urls = s.split(',').flat_map(Url::parse).collect();

        Ok(Self(urls))
    }
}
