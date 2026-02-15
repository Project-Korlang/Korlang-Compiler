use std::env;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[32mKorlang Toolchain Manager (korup)\x1b[0m");
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "update" {
        update()?;
    } else {
        println!("Usage: korup update");
    }
    
    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("korup/0.1.0")
        .build()?;
    
    println!("Checking for updates...");
    let releases: Vec<Release> = client
        .get("https://api.github.com/repos/Project-Korlang/Korlang-Site/releases")
        .send()?
        .json()?;
    
    if releases.is_empty() {
        println!("No releases found.");
        return Ok(());
    }
    
    let latest = &releases[0];
    println!("Latest version: {}", latest.tag_name);
    
    let target_os = if cfg!(target_os = "windows") { "windows" } 
                    else if cfg!(target_os = "macos") { "macos" } 
                    else { "linux" };
    
    let asset = latest.assets.iter().find(|a| a.name.contains(target_os) && (a.name.ends_with(".zip") || a.name.ends_with(".tar.gz")));
    
    if let Some(asset) = asset {
        println!("Found asset: {}", asset.name);
        download_and_install(&client, &asset.browser_download_url, &asset.name)?;
    } else {
        println!("No compatible asset found for your platform ({})", target_os);
    }
    
    Ok(())
}

fn download_and_install(client: &Client, url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = client.get(url).send()?;
    let total_size = response.content_length().unwrap_or(0);
    
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>- "));

    let mut buffer = Vec::new();
    let mut downloaded = 0;
    let mut chunk = vec![0u8; 8192];
    
    use std::io::Read;
    while let Ok(n) = response.read(&mut chunk) {
        if n == 0 { break; }
        buffer.extend_from_slice(&chunk[..n]);
        downloaded += n as u64;
        pb.set_position(downloaded);
    }
    pb.finish_with_message("Download complete");

    println!("Installing to /home/ns/.korlang/bin...");
    // Extraction logic (platform specific) would go here
    // For now, we inform the user it needs to be integrated with the OS path
    
    Ok(())
}
