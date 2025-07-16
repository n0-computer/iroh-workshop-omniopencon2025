use std::{env, str::FromStr};

use anyhow::{Context, Result};
use iroh::Watcher;
use iroh_base::SecretKey;
use n0_future::StreamExt;
use rand::thread_rng;

/// Gets a secret key from the IROH_SECRET environment variable or generates a new random one.
/// If the environment variable is set, it must be a valid string representation of a secret key.
pub fn get_or_generate_secret_key() -> Result<SecretKey> {
    if let Ok(secret) = env::var("IROH_SECRET") {
        // Parse the secret key from string
        SecretKey::from_str(&secret).context("Invalid secret key format")
    } else {
        // Generate a new random key
        let secret_key = SecretKey::generate(&mut thread_rng());
        println!(
            "Generated new secret key: {}",
            hex::encode(secret_key.to_bytes())
        );
        println!("To reuse this key, set the IROH_SECRET environment variable to this value");
        Ok(secret_key)
    }
}

pub async fn await_relay(ep: &iroh::Endpoint) -> iroh::NodeAddr {
    let mut stream = ep.node_addr().stream_updates_only();
    loop {
        if let Some(Some(addr)) = stream.next().await {
            if addr.relay_url.is_some() {
                return addr;
            }
        }
    }
}
