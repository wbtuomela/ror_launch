extern crate base64;
extern crate bytebuffer;
extern crate curl;
#[macro_use]
extern crate log;
extern crate md5;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs as xml;
extern crate sha2;
extern crate simple_logger;

mod client;
mod enums;
mod launcher;
mod patcher;

use client::Client;
use launcher::{get_launcher_url, Launcher};
use patcher::Patcher;
use std::env;
use std::process::Command;

#[cfg(debug_assertions)]
use log::LogLevel;

/// Entry point
fn main() {
    #[cfg(debug_assertions)]
    simple_logger::init_with_level(LogLevel::Info).unwrap();

    println!("👾 command line Return of Reckoning launcher (by Sammy)");

    info!(
        "[1/7] Fetching launcher information from {}",
        get_launcher_url()
    );

    let launcher_info = Launcher::new();

    info!(
        "[2/7] auth server is at {}:{}",
        &launcher_info.get_ip(),
        &launcher_info.get_port()
    );

    info!("[3/7] attempting to open socket to auth server... ");
    let mut client = Client::new(launcher_info.get_ip(), launcher_info.get_port());
    info!("[4/7] connection established.");

    client.check();

    auth(&mut client);
}

/// attempt to send auth information to the auth server
/// packet format is:
/// 0x0, 0x0, 0x0, 0x0, 0x3 -- this seems to specify auth, we expect 0x4 in response
/// [[username bytes]]
/// [[(hashed)username:password]]
/// [[packet size]]
fn auth(client: &mut Client) {
    info!("[5/7] sending auth info...");
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 3);

    let cmd_user = &args[1];
    let cmd_pass = &args[2];

    client.auth(cmd_user, cmd_pass, |client| {
        patch_warhammer(client);
    });
}

fn patch_warhammer(client: &Client) {
    info!("[6/7] Determining if WAR.exe needs a patch.");
    let mut patcher = Patcher::new();
    if patcher.needs_patch() {
        debug!("... client requires patching");
        patcher.patch();
    } else {
        debug!("... no patch needed");
    }

    start_warhammer(client);
}

/// finally attempt to start the warhammer executable with the provided
/// account username encoded via base64 along with the auth result that
/// we should recieve from the auth server encoded via base64
fn start_warhammer(client: &Client) {
    info!("[7/7] Starting Warhammer Online - Age of Reckoning");
    println!("Welcome to Return of Reckoning! 😈");

    #[cfg(target_family = "unix")]
    let _war_proc = Command::new("wine")
        .arg("WAR.exe")
        .arg(format!("--acctname={}", base64::encode(client.get_user())))
        .arg(format!("--sesstoken={}", base64::encode(client.get_auth())))
        .output()
        .expect("failed to start WAR.exe");

    #[cfg(target_os = "windows")]
    let _war_proc = Command::new("WAR.exe")
        .arg(format!("--acctname={}", base64::encode(client.get_user())))
        .arg(format!("--sesstoken={}", base64::encode(client.get_auth())))
        .output()
        .expect("failed to start WAR.exe");
}
