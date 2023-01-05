use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::time::Duration;

use log::{error, info, warn};
use rog_anime::error::AnimeError;
use rog_anime::{ActionData, ActionLoader, AnimTime, AnimeType, Fade, Vec2};
use serde_derive::{Deserialize, Serialize};

use crate::{config_file_open, VERSION};

pub static CONFIG_FILE: &str = "anime.conf";
pub static ANIME_CACHE_PATH: &str = "/etc/asusd/anime-cache.conf";

#[derive(Deserialize, Serialize)]
pub struct AnimeConfigV341 {
    pub system: Option<ActionLoader>,
    pub boot: Option<ActionLoader>,
    pub suspend: Option<ActionLoader>,
    pub shutdown: Option<ActionLoader>,
}

impl AnimeConfigV341 {
    pub(crate) fn into_current(self) -> AnimeConfig {
        AnimeConfig {
            system: if let Some(ani) = self.system {
                vec![ani]
            } else {
                vec![]
            },
            boot: if let Some(ani) = self.boot {
                vec![ani]
            } else {
                vec![]
            },
            wake: if let Some(ani) = self.suspend {
                vec![ani]
            } else {
                vec![]
            },
            shutdown: if let Some(ani) = self.shutdown {
                vec![ani]
            } else {
                vec![]
            },
            brightness: 1.0,
            awake_enabled: true,
            boot_anim_enabled: true,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AnimeConfigV352 {
    pub system: Vec<ActionLoader>,
    pub boot: Vec<ActionLoader>,
    pub wake: Vec<ActionLoader>,
    pub shutdown: Vec<ActionLoader>,
    pub brightness: f32,
}

impl AnimeConfigV352 {
    pub(crate) fn into_current(self) -> AnimeConfig {
        AnimeConfig {
            system: self.system,
            boot: self.boot,
            wake: self.wake,
            shutdown: self.shutdown,
            brightness: 1.0,
            awake_enabled: true,
            boot_anim_enabled: true,
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct AnimeConfigCached {
    pub system: Vec<ActionData>,
    pub boot: Vec<ActionData>,
    pub wake: Vec<ActionData>,
    pub shutdown: Vec<ActionData>,
}

impl AnimeConfigCached {
    pub fn init_from_config(
        &mut self,
        config: &AnimeConfig,
        anime_type: AnimeType,
    ) -> Result<(), AnimeError> {
        let mut sys = Vec::with_capacity(config.system.len());
        for ani in &config.system {
            sys.push(ActionData::from_anime_action(anime_type, ani)?);
        }
        self.system = sys;

        let mut boot = Vec::with_capacity(config.boot.len());
        for ani in &config.boot {
            boot.push(ActionData::from_anime_action(anime_type, ani)?);
        }
        self.boot = boot;

        let mut wake = Vec::with_capacity(config.wake.len());
        for ani in &config.wake {
            wake.push(ActionData::from_anime_action(anime_type, ani)?);
        }
        self.wake = wake;

        let mut shutdown = Vec::with_capacity(config.shutdown.len());
        for ani in &config.shutdown {
            shutdown.push(ActionData::from_anime_action(anime_type, ani)?);
        }
        self.shutdown = shutdown;
        Ok(())
    }
}

/// Config for base system actions for the anime display
#[derive(Deserialize, Serialize)]
pub struct AnimeConfig {
    pub system: Vec<ActionLoader>,
    pub boot: Vec<ActionLoader>,
    pub wake: Vec<ActionLoader>,
    pub shutdown: Vec<ActionLoader>,
    pub brightness: f32,
    pub awake_enabled: bool,
    pub boot_anim_enabled: bool,
}

impl Default for AnimeConfig {
    fn default() -> Self {
        AnimeConfig {
            system: Vec::new(),
            boot: Vec::new(),
            wake: Vec::new(),
            shutdown: Vec::new(),
            brightness: 1.0,
            awake_enabled: true,
            boot_anim_enabled: true,
        }
    }
}

impl AnimeConfig {
    /// `load` will attempt to read the config, and panic if the dir is missing
    pub fn load() -> Self {
        let mut file = config_file_open(CONFIG_FILE);
        let mut buf = String::new();
        if let Ok(read_len) = file.read_to_string(&mut buf) {
            if read_len == 0 {
                return AnimeConfig::create_default(&mut file);
            } else {
                if let Ok(mut data) = serde_json::from_str(&buf) {
                    Self::clamp_config_brightness(&mut data);
                    return data;
                } else if let Ok(data) = serde_json::from_str::<AnimeConfigV341>(&buf) {
                    let mut config = data.into_current();
                    config.write();
                    info!("Updated config version to: {}", VERSION);
                    Self::clamp_config_brightness(&mut config);
                    return config;
                } else if let Ok(data) = serde_json::from_str::<AnimeConfigV352>(&buf) {
                    let mut config = data.into_current();
                    config.write();
                    info!("Updated config version to: {}", VERSION);
                    Self::clamp_config_brightness(&mut config);
                    return config;
                }
                warn!(
                    "Could not deserialise {}.\nWill rename to {}-old and recreate config",
                    CONFIG_FILE, CONFIG_FILE
                );
                let cfg_old = CONFIG_FILE.to_string() + "-old";
                std::fs::rename(CONFIG_FILE, cfg_old).unwrap_or_else(|err| {
                    panic!(
                        "Could not rename. Please remove {} then restart service: Error {}",
                        CONFIG_FILE, err
                    )
                });
            }
        }
        AnimeConfig::create_default(&mut file)
    }

    fn clamp_config_brightness(mut config: &mut AnimeConfig) {
        if config.brightness < 0.0 || config.brightness > 1.0 {
            warn!(
                "Clamped brightness to [0.0 ; 1.0], was {}",
                config.brightness
            );
            config.brightness = f32::max(0.0, f32::min(1.0, config.brightness));
        }
    }

    fn create_default(file: &mut File) -> Self {
        // create a default config here
        let config = AnimeConfig {
            system: vec![],
            boot: vec![ActionLoader::ImageAnimation {
                file: "/usr/share/asusd/anime/custom/sonic-run.gif".into(),
                scale: 0.9,
                angle: 0.65,
                translation: Vec2::default(),
                brightness: 1.0,
                time: AnimTime::Fade(Fade::new(
                    Duration::from_secs(2),
                    Some(Duration::from_secs(2)),
                    Duration::from_secs(2),
                )),
            }],
            wake: vec![ActionLoader::ImageAnimation {
                file: "/usr/share/asusd/anime/custom/sonic-run.gif".into(),
                scale: 0.9,
                angle: 0.65,
                translation: Vec2::default(),
                brightness: 1.0,
                time: AnimTime::Fade(Fade::new(
                    Duration::from_secs(2),
                    Some(Duration::from_secs(2)),
                    Duration::from_secs(2),
                )),
            }],
            shutdown: vec![ActionLoader::ImageAnimation {
                file: "/usr/share/asusd/anime/custom/sonic-wait.gif".into(),
                scale: 0.9,
                angle: 0.0,
                translation: Vec2::new(3.0, 2.0),
                brightness: 1.0,
                time: AnimTime::Infinite,
            }],
            brightness: 1.0,
            awake_enabled: true,
            boot_anim_enabled: true,
        };
        // Should be okay to unwrap this as is since it is a Default
        let json = serde_json::to_string_pretty(&config).unwrap();
        file.write_all(json.as_bytes())
            .unwrap_or_else(|_| panic!("Could not write {}", CONFIG_FILE));
        config
    }

    pub fn read(&mut self) {
        let mut file = OpenOptions::new()
            .read(true)
            .open(CONFIG_FILE)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", CONFIG_FILE, err));
        let mut buf = String::new();
        if let Ok(l) = file.read_to_string(&mut buf) {
            if l == 0 {
                warn!("File is empty {}", CONFIG_FILE);
            } else {
                let x: AnimeConfig = serde_json::from_str(&buf)
                    .unwrap_or_else(|_| panic!("Could not deserialise {}", CONFIG_FILE));
                *self = x;
            }
        }
    }

    pub fn write(&self) {
        let mut file = File::create(CONFIG_FILE).expect("Couldn't overwrite config");
        let json = serde_json::to_string_pretty(self).expect("Parse config to JSON failed");
        file.write_all(json.as_bytes())
            .unwrap_or_else(|err| error!("Could not write config: {}", err));
    }
}
