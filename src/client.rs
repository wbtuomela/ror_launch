use enums::ELanguage;
use std::io::prelude::*;
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::io::{self, BufReader, BufWriter, Error, Read, Result, Write};

#[derive(Debug)]
pub struct Client {
    user: String,
    auth: String,
    lang: ELanguage,
    stream: TcpStream,
}

impl Client {
    pub fn new(ip: &String, port: &String) -> Self {
        let formatted_addr = format!("{}:{}", ip, port);
        let addr = formatted_addr.parse::<SocketAddr>().unwrap();

        Self {
            user: String::default(),
            auth: String::default(),
            lang: ELanguage::English,
            stream: TcpStream::connect(&addr).unwrap(),
        }
    }

    pub fn update_language(&mut self, new_lang: ELanguage) {
        self.lang = new_lang;
    }

    pub fn disconnect(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }

    pub fn set_auth<'a>(&mut self, new_auth: &'a str) {
        self.auth = String::from(new_auth);
    }

    pub fn get_auth(&self) -> &String {
        &self.auth
    }

    pub fn set_user<'a>(&mut self, new_user: &'a str) {
        self.user = String::from(new_user);
    }

    pub fn get_user(&self) -> &String {
        &self.user
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data)?;
        self.stream.flush()
    }

    pub fn peek(&mut self, data: &mut [u8]) -> Result<usize> {
        self.stream.peek(data)
    }

    pub fn read(&mut self, dest: &mut [u8]) -> Result<usize> {
        self.stream.read(dest)
    }
}
