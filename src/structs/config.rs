use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use serde::Deserialize;
use toml::Value;

use crate::{consts::template_files::MRVILLAGE_CONFIG, traits::merge::Merge};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub ssh: Ssh,
    #[serde(default)]
    pub templates: Templates,
}

impl Config {
    pub fn load() -> Result<Self> {
        Self::load_from_dir(std::env::current_dir()?)
    }

    pub fn load_from_dir(dir: PathBuf) -> Result<Self> {
        let mut config = Self::load_global();
        let mut configs = Self::configs(dir);
        while let Some(c) = configs.pop_front() {
            config.merge(&c);
        }
        Ok(config)
    }

    // closest to the front of the queue are at the lowest point in the hierarchy, i.e. have least precedence when determining attributes
    pub fn config_paths(mut dir: PathBuf) -> VecDeque<PathBuf> {
        let mut paths = VecDeque::new();
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
    pub fn configs(dir: PathBuf) -> VecDeque<Config> {
        let mut configs = VecDeque::new();
        let mut paths = Self::config_paths(dir);
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
        let config = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config).unwrap()
    }
}

impl Merge<'_> for Config {
    fn merge(&mut self, other: &Self) {
        self.ssh.merge(&other.ssh);
        self.templates.merge(&other.templates);
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Ssh {
    #[serde(default)]
    pub hosts: HashMap<String, Host>,
}

impl Ssh {
    pub fn get_host_with_root(&mut self, name: &str) -> Option<&Host> {
        if let Some(host) = self.hosts.get_mut(name) {
            if host.root_password.is_none() {
                host.prompt_for_root_if_none(name);
            }
            Some(host)
        } else {
            None
        }
    }

    pub fn get_host_with_root_res(&mut self, name: &str) -> Result<&Host> {
        match self.get_host_with_root(name) {
            Some(host) => Ok(host),
            None => Err(anyhow::anyhow!("SSH host {} not found", name)),
        }
    }
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

#[derive(Clone, Debug, Deserialize)]
pub struct Host {
    pub ip: std::net::IpAddr,
    #[serde(default = "Host::default_port")]
    pub port: u16,
    pub user: String,
    pub root_password: Option<String>,
    #[serde(default = "Host::default_forward_agent")]
    pub forward_agent: bool,
}

impl Host {
    #[inline(always)]
    fn default_port() -> u16 {
        22
    }

    #[inline(always)]
    fn default_forward_agent() -> bool {
        false
    }

    pub fn connection_string(&self) -> String {
        format!("{}@{}", self.user, self.ip)
    }

    pub fn prompt_for_root_if_none(&mut self, name: &str) {
        if self.root_password.is_none() {
            let mut root_password = String::new();
            print!("Enter root password for {}: ", name);
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut root_password).unwrap();
            self.root_password = Some(root_password);
        }
    }
}

impl Merge<'_> for Host {
    fn merge(&mut self, other: &Self) {
        self.ip = other.ip;
        self.port = other.port;
        self.user = other.user.clone();
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Templates {
    pub vars: HashMap<String, Value>,
}

impl Merge<'_> for Templates {
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.vars {
            self.vars.insert(k.clone(), v.to_string().into());
        }
    }
}
