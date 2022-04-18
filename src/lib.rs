pub mod mccs;

use std::io::{Error, ErrorKind};

#[cfg(feature = "node-bindings")]
mod neon_bindings;

#[cfg(feature = "node-bindings")]
use neon_bindings::{display_get_brightness, display_set_brightness, display_info};
#[cfg(feature = "node-bindings")]
use neon::prelude::*;

use ddc::{Ddc, VcpValue};
use ddc_hi::Display;

pub struct EnhancedDisplay {
    pub inner_display: Display,
    pub id: usize
}

#[cfg(feature = "node-bindings")]
impl Finalize for EnhancedDisplay {}

pub fn get_enhanced_displays() -> impl Iterator<Item = EnhancedDisplay> {
    Display::enumerate().into_iter().enumerate().map(|(index, mut display)|
        match display.update_capabilities() {
            Ok(()) => Ok(EnhancedDisplay {
                inner_display: display,
                id: index
            }),
            Err(err) => Err(Error::new(ErrorKind::TimedOut, err.to_string()))
        }
    ).filter_map(|display| display.ok())
}

pub fn get_brightness(id: usize) -> Result<VcpValue, Error> {
    Display::enumerate()
        .into_iter()
        .enumerate()
        .find(|(index, _)| index.clone() == id)
        .ok_or(Error::new(ErrorKind::Unsupported, format!("There is no display with id: {}", id)))
        .map(|(_, mut display)|
            match display.update_capabilities() {
                Ok(()) => match display.info.mccs_database.get(mccs::ImageAdjustment::Luminance.into()) {
                    Some(feature) => {
                        display.handle.get_vcp_feature(feature.code)
                            .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))
                    }
                    None => Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations"))
                },
                Err(err) => Err(Error::new(ErrorKind::TimedOut, err.to_string()))
            }
        )?
}

pub fn set_brightness(id: usize, value: u16) -> Result<(), Error> {
    Display::enumerate()
        .into_iter()
        .enumerate()
        .find(|(index, _)| index.clone() == id)
        .ok_or(Error::new(ErrorKind::Unsupported, format!("There is no display with id: {}", id)))
        .map(|(_, mut display)|
            match display.update_capabilities() {
                Ok(()) => match display.info.mccs_database.get(mccs::ImageAdjustment::Luminance.into()) {
                    Some(feature) => {
                        display.handle.set_vcp_feature(feature.code, value)
                            .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))
                    }
                    None => Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations"))
                },
                Err(err) => Err(Error::new(ErrorKind::TimedOut, err.to_string()))
            }
        )?
}

#[cfg(feature = "node-bindings")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("display_info", display_info)?;
    cx.export_function("display_get_brightness", display_get_brightness)?;
    cx.export_function("display_set_brightness", display_set_brightness)?;
    Ok(())
}