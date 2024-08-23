use reqwest::blocking::Client;
use std::{fs::File, process::Command};

static PATH: &str = "Minecraft.jar";
static URL: &str = "http://s3.amazonaws.com/Minecraft.Download/launcher/Minecraft.jar";
static USER_AGENT: &str = "ShayBox/USB-MC";
static WORK_DIR: &str = ".minecraft";

fn main() -> anyhow::Result<()> {
    let path = std::env::var("PATH").unwrap_or_else(|_| PATH.to_string());
    let url = std::env::var("URL").unwrap_or_else(|_| URL.to_string());
    let user_agent = std::env::var("USER_AGENT").unwrap_or_else(|_| USER_AGENT.to_string());
    let work_dir = std::env::var("WORK_DIR").unwrap_or_else(|_| WORK_DIR.to_string());
    let client = Client::builder().user_agent(user_agent).build()?;
    std::fs::create_dir_all(&work_dir)?;

    let path = format!("{work_dir}/{path}");
    let mut resp = client.get(url).send()?;
    let mut file = File::create(&path)?;
    resp.copy_to(&mut file)?;

    Command::new("java")
        .args(["-jar", &path, "--workDir", &work_dir])
        .args(std::env::args())
        .spawn()?
        .wait()?;

    Ok(())
}
