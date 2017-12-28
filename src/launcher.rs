use curl::easy::Easy;
use xml::deserialize;

/// This url contains an xml file with information about where the auth server is
const LAUNCHER_URL: &str = "http://launcher.returnofreckoning.com/launcher.xml";

/// Helper function to get at `LAUNCHER_URL`
pub fn get_launcher_url() -> &'static str {
    LAUNCHER_URL
}

/// The xml body contains a sub element called `LauncherServer`
#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherServer {
    #[serde(rename = "Ip")] ip: String,
    #[serde(rename = "Port")] port: String,
}

/// Main xml body for deserialization via serde
#[derive(Debug, Serialize, Deserialize)]
pub struct Launcher {
    #[serde(rename = "Version")] version: String,
    #[serde(rename = "PasswordMode")] password_mode: String,
    #[serde(rename = "LauncherServer")] launcher_server: LauncherServer,
}

impl Launcher {
    /// create new launcher info type
    /// sends a curl request to download xml information about where the auth serv is
    pub fn new() -> Self {
        let mut dest = Vec::new();
        {
            let mut easy = Easy::new();
            easy.ssl_verify_peer(false).unwrap();
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

    /// get the auth serv ip as a string
    pub fn get_ip(&self) -> &String {
        &self.launcher_server.ip
    }

    /// get the auth serv port as a string
    pub fn get_port(&self) -> &String {
        &self.launcher_server.port
    }
}
