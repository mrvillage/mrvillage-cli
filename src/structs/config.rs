use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

use serde::Deserialize;

use crate::{consts::template_files::MRVILLAGE_CONFIG, traits::merge::Merge};

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub ssh: Ssh,
}

impl Config {
    pub fn load() -> Self {
        let mut global_config = Self::load_global();
        let mut configs = Self::configs();
        while let Some(config) = configs.pop_front() {
            global_config.merge(&config);
        }
        global_config
    }

    // closest to the front of the queue are at the lowest point in the hierarchy, i.e. have least precedence when determining attributes
    pub fn config_paths() -> VecDeque<PathBuf> {
        let mut paths = VecDeque::new();
        let mut dir = std::env::current_dir().unwrap();
        loop {
            let mut path = dir.clone();
            path.push(".mrvillage.toml");
            if path.exists() {
                paths.push_front(path);
            }
            if !dir.pop() {
                break;
            }
        }
        paths
    }

    // closest to the front of the queue are at the lowest point in the hierarchy, i.e. have least precedence when determining attributes
    pub fn configs() -> VecDeque<Config> {
        let mut configs = VecDeque::new();
        let mut paths = Self::config_paths();
        while let Some(path) = paths.pop_back() {
            configs.push_front(Self::read_from_file(&path));
        }
        configs
    }

    pub fn global_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".mrvillage");
        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
        }
        path.push("config.toml");
        if !path.exists() {
            std::fs::write(&path, MRVILLAGE_CONFIG.content).unwrap();
        }
        path
    }

    pub fn load_global() -> Self {
        Self::read_from_file(&Self::global_path())
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

#[derive(Clone, Default, Deserialize)]
pub struct Ssh {
    #[serde(default)]
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
    #[serde(default = "Host::default_port")]
    pub port: u16,
    pub user: String,
    pub root_password: Option<String>,
}

impl Host {
    fn default_port() -> u16 {
        22
    }
}

impl Merge<'_> for Host {
    fn merge(&mut self, other: &Self) {
        self.host = other.host;
        self.port = other.port;
        self.user = other.user.clone();
    }
}
