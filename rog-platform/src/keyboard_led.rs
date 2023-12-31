use std::path::PathBuf;

use log::{info, warn};

use crate::error::{PlatformError, Result};
use crate::{attr_u8, has_attr, set_attr_u8_array, to_device};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Clone)]
pub struct KeyboardLed {
    path: PathBuf,
}

impl KeyboardLed {
    attr_u8!("brightness", path);

    has_attr!("kbd_rgb_mode" path);

    set_attr_u8_array!(
        /// kbd_rgb_mode can only be set, not read back
        "kbd_rgb_mode"
        path
    );

    has_attr!("kbd_rgb_state" path);

    set_attr_u8_array!(
        /// kbd_rgb_state can only be set, not read back
        "kbd_rgb_state"
        path
    );

    pub fn new() -> Result<Self> {
        let mut enumerator = udev::Enumerator::new().map_err(|err| {
            warn!("{}", err);
            PlatformError::Udev("enumerator failed".into(), err)
        })?;

        enumerator.match_subsystem("leds").map_err(|err| {
            warn!("{}", err);
            PlatformError::Udev("match_subsystem failed".into(), err)
        })?;

        enumerator
            .match_sysname("asus::kbd_backlight")
            .map_err(|err| {
                warn!("{}", err);
                PlatformError::Udev("match_subsystem failed".into(), err)
            })?;

        if let Some(device) = (enumerator.scan_devices().map_err(|err| {
            warn!("{}", err);
            PlatformError::Udev("scan_devices failed".into(), err)
        })?)
        .next()
        {
            info!("Found keyboard LED controls at {:?}", device.sysname());
            return Ok(Self {
                path: device.syspath().to_owned(),
            });
        }
        Err(PlatformError::MissingFunction(
            "asus::kbd_backlight not found".into(),
        ))
    }
}
