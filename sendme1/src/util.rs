use std::{env, path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use iroh::Watcher;
use iroh_base::SecretKey;
use iroh_blobs::{
    api::remote::{GetProgress, GetProgressItem},
    get::Stats,
    HashAndFormat,
};
use n0_future::StreamExt;
use rand::{thread_rng, Rng};

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

/// Create a unique directory for sending files.
pub fn create_send_dir() -> Result<PathBuf> {
    let suffix = rand::thread_rng().gen::<[u8; 16]>();
    let cwd = std::env::current_dir()?;
    let blobs_data_dir = cwd.join(format!(".{}-send-{}", crate_name(), hex::encode(suffix)));
    if blobs_data_dir.exists() {
        println!(
            "can not share twice from the same directory: {}",
            cwd.display(),
        );
        std::process::exit(1);
    }
    Ok(blobs_data_dir)
}

pub fn create_recv_dir(content: HashAndFormat) -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let blobs_data_dir = cwd.join(format!(".{}-recv-{}", crate_name(), content));
    Ok(blobs_data_dir)
}

pub fn crate_name() -> &'static str {
    env!("CARGO_CRATE_NAME")
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

pub async fn show_fetch_progress(response: GetProgress) -> Result<Stats> {
    let mut stream = response.stream();
    loop {
        match stream.next().await {
            Some(GetProgressItem::Progress(value)) => {
                print!("\rProgress: {value}");
            }
            Some(GetProgressItem::Error(e)) => {
                return Err(e.into());
            }
            Some(GetProgressItem::Done(stats)) => {
                return Ok(stats);
            }
            None => anyhow::bail!("Stream ended unexpectedly"),
        }
    }
}
