use crate::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use ureq::{json, serde::Serialize, Agent, AgentBuilder};

lazy_static! {
    // Ureq Agent
    pub static ref UREQ_AGENT: Agent = AgentBuilder::new()
        .timeout(Duration::from_secs(3600))
        .build();
}

pub fn send_discord_message(message: impl Serialize) -> Result<(), Box<dyn Error>> {
    // Struct a message to Discord
    let value = json!({
        "content": "",
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

#[macro_export]
/// Load all environment variables with their respective types
macro_rules! load_env_vars {
    ($($var_name:ident : $type:ty),*) => {
        lazy_static! {
            $(
                pub static ref $var_name: $type = {
                    dotenv().ok();

                    let var_value = dotenv::var(stringify!($var_name)).unwrap_or_else(|_| panic!("{} NOT FOUND", stringify!($var_name)));
                    var_value.parse::<$type>().unwrap_or_else(|_| panic!("{} IS NOT A {}", stringify!($var_name), stringify!($type)))
                };
            )*
        }

        /// Makes sure all the environment variables are loaded
        pub fn init_env_vars() {
            $(
                lazy_static::initialize(&$var_name);
            )*
        }
    };
}

/// Chains
pub struct Chains(pub Vec<Url>);
impl FromStr for Chains {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chains = s
            .split(',')
            .map(|chain| Url::from_str(chain).map_err(|e| e.into()))
            .collect::<Result<Vec<Url>, Box<dyn Error>>>()?;

        Ok(Self(chains))
    }
}

// Load all environment variables with their respective types
load_env_vars!(
    DISCORD_WEBHOOK: Url,
    CHAINS: Chains
);
