pub mod mccs;

use std::io::{Error, ErrorKind};

#[cfg(feature = "node-bindings")]
mod neon_bindings;

#[cfg(feature = "node-bindings")]
use neon_bindings::{display_get_brightness, display_set_brightness,
                    display_info, display_get_by_id};
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
    Display::enumerate().into_iter().enumerate().map(|(id, mut display)|
        match display.update_capabilities() {
            Ok(()) => Ok(EnhancedDisplay {
                inner_display: display,
                id
            }),
            Err(err) => Err(Error::new(ErrorKind::TimedOut, err.to_string()))
        }
    ).filter_map(|display| display.ok())
}

pub fn get_enhanced_display_by_id(id: usize) -> Result<EnhancedDisplay, Error> {
    get_enhanced_displays().find(|enhanced_display| enhanced_display.id == id)
        .ok_or(Error::new(ErrorKind::Unsupported, format!("There is no display with id: {}", id)))
}

pub fn get_brightness(id: usize) -> Result<VcpValue, Error> {
    let mut enhanced_display = get_enhanced_display_by_id(id)?;
    match enhanced_display.inner_display.info.mccs_database.get(mccs::ImageAdjustment::Luminance.into()) {
        Some(feature) => {
            enhanced_display.inner_display.handle.get_vcp_feature(feature.code)
                .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))
        }
        None => Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations"))
    }
}

pub fn set_brightness(id: usize, value: u16) -> Result<(), Error> {
    let mut enhanced_display = get_enhanced_display_by_id(id)?;
    match enhanced_display.inner_display.info.mccs_database.get(mccs::ImageAdjustment::Luminance.into()) {
        Some(feature) => {
            enhanced_display.inner_display.handle.set_vcp_feature(feature.code, value)
                .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))
        }
        None => Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations"))
    }
}

#[cfg(feature = "node-bindings")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("display_info", display_info)?;
    cx.export_function("display_get_by_id", display_get_by_id)?;
    cx.export_function("display_get_brightness", display_get_brightness)?;
    cx.export_function("display_set_brightness", display_set_brightness)?;
    Ok(())
}