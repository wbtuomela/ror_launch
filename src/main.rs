extern crate base64;
extern crate bytebuffer;
extern crate curl;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs as xml;
extern crate sha2;
extern crate simple_logger;

mod client;
mod enums;
mod launcher;

use client::Client;
use launcher::{get_launcher_url, Launcher};
use bytebuffer::ByteBuffer;
use sha2::{Digest, Sha256};
use std::env;
use std::process::Command;

#[cfg(debug_assertions)]
use log::LogLevel;

fn hash_str_orig<'a>(src: &'a str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(src.as_bytes());

    let hashed_pass = hasher.result();
    hashed_pass.into_iter().collect()
}

fn hash_str_fmt<'a>(src: &'a str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(src.as_bytes());

    let hashed_pass = hasher.result();
    let mut result_str = String::new();

    for byte in hashed_pass.iter() {
        result_str.push_str(&format!("{:x}", byte));
    }

    result_str.into_bytes()
}

fn main() {
    #[cfg(debug_assertions)]
    simple_logger::init().unwrap();

    println!("👾 command line Return of Reckoning launcher (by Sammy)");

    info!(
        "[1/6] Fetching launcher information from {}",
        get_launcher_url()
    );

    let launcher_info = Launcher::new();

    info!(
        "[2/6] auth server is at {}:{}",
        &launcher_info.get_ip(),
        &launcher_info.get_port()
    );

    info!("[3/6] attempting to open socket to auth server... ");
    let mut client = Client::new(&launcher_info.get_ip(), &launcher_info.get_port());
    info!("[4/6] connection established.");

    auth(&mut client);
}

fn auth(client: &mut Client) {
    info!("[5/6] sending auth info...");
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 3);

    let cmd_user = &args[1];
    let cmd_pass = &args[2];

    client.set_user(cmd_user);

    let mut buffer = ByteBuffer::new();
    buffer.write_bytes(&[0x00, 0x00, 0x00, 0x00, 0x03]);
    buffer.write_string(cmd_user);

    {
        let pass_to_hash = format!("{}:{}", cmd_user, cmd_pass); // username:password
        let hashed_pass = hash_str_orig(&pass_to_hash);

        buffer.write_u32(hashed_pass.len() as u32);
        buffer.write_bytes(&hashed_pass);
    }

    {
        //let packet_size = buffer.len() as u32;
        //buffer.write_u32(packet_size - 1); // exclude opcode
    }

    client.send(&buffer.to_bytes()).unwrap();

    let mut temp = [0; 256];
    let read = client.read(&mut temp).unwrap();

    let mut response_buf = ByteBuffer::from_bytes(&temp[..read]);
    let _size = response_buf.read_u32();
    let opcode = response_buf.read_u8();

    match opcode {
        4 => {
            let resp = response_buf.read_u8();
            match resp {
                1u8 => panic!("invalid username / password"),
                2u8 => panic!("account suspended"),
                3u8 => panic!("account inactive"),
                _ => {}
            }

            let auth = response_buf.read_string();
            client.set_auth(&auth);

            println!("Welcome to WAR!! 😈");
            start_warhammer(&client);
        }
        _ => {}
    }
}

fn start_warhammer(client: &Client) {
    info!("[6/6] Starting Warhammer Online - Age of Reckoning");

    #[cfg(target_os = "macos")]
    let war_proc = Command::new("wine")
        .arg("WAR.exe")
        .arg(format!(
            "--accountname={}",
            base64::encode(client.get_user())
        ))
        .arg(format!("--sesstoken={}", base64::encode(client.get_auth())))
        .output()
        .expect("failed to start WAR.exe");

    #[cfg(target_os = "windows")]
    let war_proc = Command::new("WAR.exe")
        .arg(format!(
            "--accountname={}",
            base64::encode(client.get_user())
        ))
        .arg(format!("--sesstoken={}", base64::encode(client.get_auth())))
        .output()
        .expect("failed to start WAR.exe");
}
