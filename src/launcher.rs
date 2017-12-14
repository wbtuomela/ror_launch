use curl::easy::Easy;
use xml::deserialize;

const LAUNCHER_URL: &'static str = "http://launcher.returnofreckoning.com/launcher.xml";

pub fn get_launcher_url() -> &'static str {
    LAUNCHER_URL
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherServer {
    #[serde(rename = "Ip")] ip: String,
    #[serde(rename = "Port")] port: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Launcher {
    #[serde(rename = "Version")] version: String,
    #[serde(rename = "PasswordMode")] password_mode: String,
    #[serde(rename = "LauncherServer")] launcher_server: LauncherServer,
}

impl Launcher {
    pub fn new() -> Self {
        let mut dest = Vec::new();
        {
            let mut easy = Easy::new();
            easy.url(LAUNCHER_URL).unwrap();
            easy.follow_location(true).unwrap();

            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    dest.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();

            transfer.perform().unwrap();
        }

        deserialize(&dest[..]).unwrap()
    }

    pub fn get_ip(&self) -> &String {
        &self.launcher_server.ip
    }

    pub fn get_port(&self) -> &String {
        &self.launcher_server.port
    }
}
