pub mod mccs;

use std::io::{Error, ErrorKind};

#[cfg(feature = "node-bindings")]
mod neon_bindings;

#[cfg(feature = "node-bindings")]
use neon_bindings::{display_get_brightness, display_set_brightness, display_info};
#[cfg(feature = "node-bindings")]
use neon::prelude::*;

use ddc::{Ddc, DdcHost, VcpValue};
use ddc_hi::{Display, Query};

pub struct EnhancedDisplay {
    pub inner_display: Display,
    pub id: usize,
}

#[cfg(feature = "node-bindings")]
impl Finalize for EnhancedDisplay {}

pub fn get_enhanced_displays(query: Query, needs_caps: bool) -> Result<Vec<EnhancedDisplay>, Error> {
    Display::enumerate().into_iter().enumerate().map(|(index, mut display)|
        if needs_caps {
            display.update_capabilities()
                .map(|_| EnhancedDisplay { id: index, inner_display: display })
                .map_err(|err| Error::new(ErrorKind::TimedOut, err.to_string()))
        } else {
            Ok(display).map(|d| EnhancedDisplay { id: index, inner_display: d })
        }
    ).filter(|display| if let &Ok(ref display) = display {
        query.matches(&display.inner_display.info)
    } else {
        true
    }).collect()
}

pub fn get_enhanced_display(query: Query, needs_caps: bool, id: usize) -> Result<EnhancedDisplay, Error> {
    get_enhanced_displays(query, needs_caps)?
        .into_iter()
        .find(|display| display.id == id)
        .map(|display| Ok(display))
        .or(Some(Err(Error::new(ErrorKind::Unsupported, format!("There is no display with id: {}", id))))).unwrap()
}

pub fn get_brightness(id: usize) -> Result<VcpValue, Error> {
    let mut display = get_enhanced_display(Query::Any, true, id)?;
    let result = display.inner_display.info.mccs_database
        .get(mccs::ImageAdjustment::Luminance.into())
        .map(|feature| display.inner_display.handle.get_vcp_feature(feature.code)
            .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string())))
        .or(Some(Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations")))).unwrap();
    display.inner_display.handle.sleep();
    result
}

pub fn set_brightness(id: usize, value: u16) -> Result<(), Error> {
    let mut display = get_enhanced_display(Query::Any, true, id)?;
    let result = display.inner_display.info.mccs_database
        .get(mccs::ImageAdjustment::Luminance.into())
        .map(|feature| display.inner_display.handle.set_vcp_feature(feature.code, value)
            .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string())))
        .or(Some(Err(Error::new(ErrorKind::Unsupported, "This display doesn't support brightness operations")))).unwrap();
    display.inner_display.handle.sleep();
    result
}

#[cfg(feature = "node-bindings")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("display_info", display_info)?;
    cx.export_function("display_get_brightness", display_get_brightness)?;
    cx.export_function("display_set_brightness", display_set_brightness)?;
    Ok(())
}