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
extern crate tempfile;

mod client;
mod enums;
mod launcher;
mod patcher;

use client::Client;
use launcher::{get_launcher_url, Launcher};
use patcher::Patcher;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::{NamedTempFile, NamedTempFileOptions};

#[cfg(debug_assertions)]
use log::Level;

/// Entry point
fn main() {
    #[cfg(debug_assertions)]
    simple_logger::init_with_level(Level::Info).unwrap();

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

    let username = args[1].clone();
    let password = args[2].clone();
    let mut prefix = String::new();
    if args.len() > 3 {
        prefix = args[3].clone();
    }

    client.auth(&username, &password, &prefix, |client| {
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

    start_warhammer(client.get_user(), client.get_auth(), client.get_prefix());
}

/// finally attempt to start the warhammer executable with the provided
/// account username encoded via base64 along with the auth result that
/// we should recieve from the auth server encoded via base64
fn start_warhammer(user: &String, auth: &String, prefix: &String) {
    info!("[7/7] Starting Warhammer Online - Age of Reckoning");
    println!("Welcome to Return of Reckoning! 😈");

    #[cfg(target_family = "unix")]
    {
        let temp_file: NamedTempFile = NamedTempFileOptions::new()
            .prefix("ror_launch")
            .suffix(".sh")
            .create()
            .unwrap();

        let mut wine_prefix = String::new();
        if !prefix.is_empty() {
            wine_prefix = format!("WINEPREFIX={}", prefix);
        }

        write!(
            temp_file.as_ref(),
            "{prefix} wine WAR.exe --acctname={name} --sesstoken={token}",
            prefix = wine_prefix,
            name = base64::encode(user),
            token = base64::encode(auth)
        ).unwrap();

        let mut process = match Command::new("sh")
            .current_dir(env::current_dir().unwrap())
            .arg(temp_file.path())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("failed to run wine: {}", why),
            Ok(process) => process,
        };

        process.wait().unwrap();

        temp_file.close().unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        let mut process = match Command::new("WAR.exe")
            .arg(format!("--acctname={}", base64::encode(user)))
            .arg(format!("--sesstoken={}", base64::encode(auth)))
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("failed to run WAR.exe: {}", why),
            Ok(process) => process,
        };

        process.wait().unwrap();
    }
}
