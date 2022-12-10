use std::collections::{HashMap, VecDeque};

use serde::Deserialize;

use crate::{consts::template_files::MRVILLAGE_CONFIG, traits::merge::Merge};

#[derive(Clone, Deserialize)]
pub struct Config {
    pub ssh: Ssh,
}

impl Config {
    pub fn load() -> Self {
        let mut global_config = Config::load_global();
        let mut configs = VecDeque::new();
        let mut dir = std::env::current_dir().unwrap();
        loop {
            let mut path = dir.clone();
            path.push(".mrvillage.toml");
            if path.exists() {
                configs.push_front(Self::read_from_file(&path));
            }
            if !dir.pop() {
                break;
            }
        }
        while let Some(config) = configs.pop_front() {
            global_config.merge(&config);
        }
        global_config
    }

    fn load_global() -> Self {
        let mut path = dirs::home_dir().unwrap();
        path.push(".mrvillage");
        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
        }
        path.push("config.toml");
        if !path.exists() {
            MRVILLAGE_CONFIG
                .write(&path, MRVILLAGE_CONFIG.content.to_string())
                .unwrap();
        }
        Self::read_from_file(&path)
    }

    fn read_from_file(path: &std::path::PathBuf) -> Self {
        let config = std::fs::read_to_string(&path).unwrap();
        toml::from_str(&config).unwrap()
    }
}

impl Merge<'_> for Config {
    fn merge(&mut self, other: &Self) {
        self.ssh.merge(&other.ssh);
    }
}

#[derive(Clone, Deserialize)]
pub struct Ssh {
    pub hosts: HashMap<String, Host>,
}

impl Merge<'_> for Ssh {
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.hosts {
            if let Some(host) = self.hosts.get_mut(k) {
                host.merge(v);
            } else {
                self.hosts.insert(k.clone(), v.clone());
            }
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Host {
    pub host: std::net::IpAddr,
    pub port: Option<u16>,
    pub user: String,
}

impl Merge<'_> for Host {
    fn merge(&mut self, other: &Self) {
        self.host = other.host;
        if let Some(port) = other.port {
            self.port = Some(port);
        }
        self.user = other.user.clone();
    }
}
