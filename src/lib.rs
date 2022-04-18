pub mod mccs;

use std::io::{Error, ErrorKind};
use std::cell::RefCell;

#[cfg(feature = "node-bindings")]
mod neon_bindings;

#[cfg(feature = "node-bindings")]
use neon_bindings::{display_get_brightness, display_set_brightness, display_info};
#[cfg(feature = "node-bindings")]
use neon::prelude::*;

use ddc::{Ddc, VcpValue};
use ddc_hi::Display;
use uuid::Uuid;

pub struct EnhancedDisplay {
    pub inner_display: Display,
    pub uuid: String
}

#[cfg(feature = "node-bindings")]
impl Finalize for EnhancedDisplay {}

thread_local! {
    pub static ENHANCED_DISPLAYS: RefCell<Vec<EnhancedDisplay>> = RefCell::new(
        Display::enumerate().into_iter().map(|mut display|
            match display.update_capabilities() {
                Ok(()) => Ok(EnhancedDisplay {
                    inner_display: display,
                    uuid: Uuid::new_v4().to_simple().to_string()
                }),
                Err(err) => Err(Error::new(ErrorKind::TimedOut, err.to_string()))
            }
        ).filter_map(|display| display.ok()).collect());
}


pub fn get_brightness(uuid: String) -> Result<VcpValue, Error> {
    ENHANCED_DISPLAYS.with(|enhanced_displays| {
        let mut enhanced_displays = enhanced_displays.borrow_mut();
        enhanced_displays.iter_mut().find(|enhanced_display|
            enhanced_display.uuid == uuid).map(|enhanced_display|
            match enhanced_display.inner_display.info.mccs_database.get(mccs::ImageAdjustment::Luminance.into()) {
                Some(feature) => {
                    enhanced_display.inner_display.handle.get_vcp_feature(feature.code)
                        .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))
                }
                None => Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations"))
            }).unwrap_or(Err(Error::new(ErrorKind::Unsupported, format!("There is no display with uuid: {}", uuid))))
    })
}

pub fn set_brightness(uuid: String, value: u16) -> Result<(), Error> {
    ENHANCED_DISPLAYS.with(|enhanced_displays| {
        let mut enhanced_displays = enhanced_displays.borrow_mut();
        enhanced_displays.iter_mut().find(|enhanced_display|
            enhanced_display.uuid == uuid).map(|enhanced_display|
            match enhanced_display.inner_display.info.mccs_database.get(mccs::ImageAdjustment::Luminance.into()) {
                Some(feature) => {
                    enhanced_display.inner_display.handle.set_vcp_feature(feature.code, value)
                        .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))
                }
                None => Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations"))
            }).unwrap_or(Err(Error::new(ErrorKind::Unsupported, format!("There is no display with uuid: {}", uuid))))
    })
}

#[cfg(feature = "node-bindings")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("display_info", display_info)?;
    cx.export_function("display_get_brightness", display_get_brightness)?;
    cx.export_function("display_set_brightness", display_set_brightness)?;
    Ok(())
}