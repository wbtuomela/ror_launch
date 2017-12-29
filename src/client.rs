use enums::{ECheckResult, EOpCode};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::io::{Read, Result, Write};
use bytebuffer::ByteBuffer;
use sha2::{Digest, Sha256};
use std::fs::{metadata, OpenOptions};
use std::path::PathBuf;
use std::env;

#[derive(Debug)]
pub struct Client {
    user: String,
    auth: String,
    prefix: String,
    stream: TcpStream,
}

impl Client {
    /// Create a new client that will open a tcp stream to the supplied ip:port
    pub fn new(ip: &str, port: &str) -> Self {
        let formatted_addr = format!("{}:{}", ip, port);
        let addr = formatted_addr.parse::<SocketAddr>().unwrap();
        let stream = TcpStream::connect(&addr).unwrap();

        Self {
            user: String::default(),
            auth: String::default(),
            prefix: String::default(),
            stream: stream,
        }
    }

    pub fn disconnect(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }

    pub fn get_auth(&self) -> &String {
        &self.auth
    }

    pub fn get_prefix(&self) -> &String {
        &self.prefix
    }

    pub fn get_user(&self) -> &String {
        &self.user
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data)?;
        self.stream.flush()
    }

    fn read(&mut self, dest: &mut [u8]) -> Result<usize> {
        self.stream.read(dest)
    }

    pub fn check(&mut self) {
        let mut path = PathBuf::new();
        path.push(env::current_dir().unwrap());
        path.push("mythloginserviceconfig.xml");

        if path.exists() {
            let meta_data = metadata(&path).unwrap();

            let mut buffer = ByteBuffer::new();
            {
                buffer.write_bytes(&[0x00, 0x00, 0x00, 0x00, EOpCode::ClCheck as u8]); // asks the server to validate our client login service confix xml
                buffer.write_u32(1u32); // client version is 1 for now
                buffer.write_u8(1u8);
                buffer.write_u64(meta_data.len());
            }

            {
                let packet_size = buffer.len() as u32;
                buffer.write_u32(packet_size - 1); // exclude opcode
            }

            self.send(&buffer.to_bytes()).unwrap();

            let mut temp = [0; 1024];
            let read = self.read(&mut temp).unwrap();

            let mut response_buf = ByteBuffer::from_bytes(&temp[..read]);
            let _size = response_buf.read_u32();
            let opcode = EOpCode::from(response_buf.read_u8());

            if opcode == EOpCode::LcrCheck {
                match ECheckResult::from(response_buf.read_u8()) {
                    ECheckResult::Success => debug!("[check]: nothing to be done."),
                    ECheckResult::Error => {
                        self.disconnect();
                        let msg = response_buf.read_string();
                        panic!("[check]: {}", msg);
                    }
                    ECheckResult::UpdateRequired => {
                        debug!("[check]: writing new login service xml.");
                        let body = response_buf.read_string();
                        let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&path)
                            .unwrap();
                        file.write_all(body.as_bytes()).unwrap();
                        file.flush().unwrap();
                    }
                    ECheckResult::Invalid => {
                        panic!("[check]: something is wrong, we got a weird response...");
                    }
                }
            }
        } else {
            error!("mythloginserviceconfig.xml does not exist");
        }
    }

    pub fn auth<F: FnOnce(&Self)>(&mut self, username: &str, passwrd: &str, prefix: &str, next: F) {
        let mut buffer = ByteBuffer::new();
        buffer.write_bytes(&[0x00, 0x00, 0x00, 0x00, EOpCode::ClStart as u8]);
        buffer.write_string(username);

        {
            let pass_to_hash = format!("{}:{}", username, passwrd); // username:password
            let hashed_pass = hash_str_fmt(&pass_to_hash);

            buffer.write_u32(hashed_pass.len() as u32);
            buffer.write_bytes(&hashed_pass);
        }

        {
            let packet_size = buffer.len() as u32;
            buffer.write_u32(packet_size - 1); // exclude opcode
        }

        self.send(&buffer.to_bytes()).unwrap();

        let mut temp = [0; 256];
        let read = self.read(&mut temp).unwrap();

        let mut response_buf = ByteBuffer::from_bytes(&temp[..read]);
        let _size = response_buf.read_u32();
        let opcode = EOpCode::from(response_buf.read_u8());

        if opcode == EOpCode::LcrStart {
            let resp = response_buf.read_u8();
            match resp {
                1u8 => panic!("invalid username / password"),
                2u8 => panic!("account suspended"),
                3u8 => panic!("account inactive"),
                _ => {}
            }

            debug!("authentication successful.");
            let auth = response_buf.read_string();
            self.auth = auth;
            self.user = String::from(username);
            self.prefix = String::from(prefix);
            next(self);
        }
    }
}

fn hash_str_fmt(src: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(src.as_bytes());

    let hashed_pass = hasher.result();
    let mut result_str = String::new();

    for byte in hashed_pass.iter() {
        result_str.push_str(&format!("{:02x}", byte));
    }

    result_str.into_bytes()
}
