pub mod mccs;

#[cfg(feature = "node-bindings")]
mod neon_bindings;
#[cfg(feature = "node-bindings")]
use neon_bindings::{display_new, display_list, display_get_brightness, display_get_contrast, display_set_brightness, display_set_contrast};
#[cfg(feature = "node-bindings")]
use neon::prelude::*;

use ddc::{Ddc, FeatureCode, VcpValue};
use ddc_hi::{DisplayInfo};
use ddc_hi::Display;

pub struct EnhancedDisplay(Display);

#[cfg(feature = "node-bindings")]
impl Finalize for EnhancedDisplay {}

impl EnhancedDisplay {
    pub fn get_value(&mut self, code: FeatureCode, error: String) -> Result<VcpValue, String> {
        match self.0.info.mccs_database.get(code) {
            Some(feature) => {
                self.0.handle.get_vcp_feature(feature.code)
                    .map_err(|error| error.to_string())
            }
            None => Err(error)
        }
    }

    pub fn set_value(&mut self, code: FeatureCode, error: String, value: u16) -> Result<(), String> {
        match self.0.info.mccs_database.get(code) {
            Some(feature) => {
                self.0.handle.set_vcp_feature(feature.code, value)
                    .map_err(|error| error.to_string())
            }
            None => Err(error)
        }
    }

    pub fn get(id: String) -> Result<EnhancedDisplay, String> {
        for mut display in Display::enumerate() {
            let option = match display.update_capabilities() {
                Ok(()) => if display.info.id == id {
                    Option::from(EnhancedDisplay(display))
                } else {
                    Option::None
                }
                Err(_) => {Option::None}
            };
            if option.is_some() {
                return Ok(option.unwrap());
            }
        }
        Err("This display doesn't exist".to_string())
    }

    pub fn list_infos() -> Vec<DisplayInfo> {
        let mut displays_info = vec![];
        for mut display in Display::enumerate() {
            match display.update_capabilities() {
                Ok(()) => displays_info.push(display.info),
                Err(_) => {}
            };
        }
        displays_info
    }

    pub fn get_brightness(&mut self) -> Result<VcpValue, String> {
        self.get_value(mccs::ImageAdjustment::Luminance.into(),
                       "This display doesn't support brightness operations".to_string())
    }

    pub fn set_brightness(&mut self, value: u16) -> Result<(), String> {
        self.set_value(mccs::ImageAdjustment::Luminance.into(),
                       "This display doesn't support brightness operations".to_string(),
                        value)
    }

    pub fn get_contrast(&mut self) -> Result<VcpValue, String> {
        self.get_value(mccs::ImageAdjustment::Contrast.into(),
                       "This display doesn't support contrast operations".to_string())
    }

    pub fn set_contrast(&mut self, value: u16) -> Result<(), String> {
        self.set_value(mccs::ImageAdjustment::Contrast.into(),
                       "This display doesn't support contrast operations".to_string(),
                       value)
    }
}

#[cfg(feature = "node-bindings")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("display_new", display_new)?;
    cx.export_function("display_list", display_list)?;
    cx.export_function("display_get_brightness", display_get_brightness)?;
    cx.export_function("display_get_contrast", display_get_contrast)?;
    cx.export_function("display_set_brightness", display_set_brightness)?;
    cx.export_function("display_set_contrast", display_set_contrast)?;
    Ok(())
}