use reqwest::blocking::Client;
use std::{fs::File, process::Command};

static WORK_DIR: &str = ".minecraft";
static USER_AGENT: &str = "ShayBox/USB-MC";
static URL: &str = "http://s3.amazonaws.com/Minecraft.Download/launcher/Minecraft.jar";
static PATH: &str = "Minecraft.jar";

fn main() {
    let work_dir = std::env::var("WORK_DIR").unwrap_or(WORK_DIR.to_string());
    std::fs::create_dir_all(&work_dir).expect("Failed to create Minecraft folder");

    let user_agent = std::env::var("USER_AGENT").unwrap_or(USER_AGENT.to_string());
    let client = Client::builder()
        .user_agent(user_agent)
        .build()
        .expect("Failed to build client");

    let path = work_dir.clone() + "/" + PATH;
    let mut resp = client.get(URL).send().expect("Failed to get response");
    let mut file = File::create(&path).expect("Failed to create file");

    resp.copy_to(&mut file).expect("Failed to write file");

    let mut child = Command::new("java")
        .args(["-jar", &path, "--workDir", &work_dir])
        .args(std::env::args())
        .spawn()
        .expect("Failed to execute process");

    child.wait().expect("Failed to wait for child process");
}
