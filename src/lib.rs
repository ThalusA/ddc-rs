mod mccs;

use ddc;
use ddc::{Ddc, FeatureCode, VcpValue};
use ddc_hi::{DisplayInfo};
pub use ddc_hi::Display;
use anyhow::{Error, anyhow};


trait EnhancedDisplay {
    fn get_value(&mut self, code: FeatureCode, error: Error) -> Result<VcpValue, Error>;
    fn set_value(&mut self, code: FeatureCode, error: Error, value: u16) -> Result<(), Error>;

    fn get(id: String) -> Option<Self> where Self: Sized;
    fn list_infos() -> Vec<DisplayInfo>;

    fn get_brightness(&mut self) -> Result<VcpValue, Error>;
    fn set_brightness(&mut self, value: u16) -> Result<(), Error>;
    fn get_contrast(&mut self) -> Result<VcpValue, Error>;
    fn set_contrast(&mut self, value: u16) -> Result<(), Error>;
}

impl EnhancedDisplay for Display {
    fn get_value(&mut self, code: FeatureCode, error: Error) -> Result<VcpValue, Error> {
        match self.info.mccs_database.get(code) {
            Some(feature) => {
                self.handle.get_vcp_feature(feature.code)
            }
            None => Err(error)
        }
    }

    fn set_value(&mut self, code: FeatureCode, error: Error, value: u16) -> Result<(), Error> {
        match self.info.mccs_database.get(code) {
            Some(feature) => {
                self.handle.set_vcp_feature(feature.code, value)
            }
            None => Err(error)
        }
    }

    fn get(id: String) -> Option<Self> {
        for mut display in Display::enumerate() {
            let option = match display.update_capabilities() {
                Ok(()) => if display.info.id == id {
                    Option::from(display)
                } else {
                    Option::None
                }
                Err(_) => {Option::None}
            };
            if option.is_some() {
                return option;
            }
        }
        Option::None
    }

    fn list_infos() -> Vec<DisplayInfo> {
        let mut displays_info = vec![];
        for mut display in Display::enumerate() {
            match display.update_capabilities() {
                Ok(()) => displays_info.push(display.info),
                Err(_) => {}
            };
        }
        displays_info
    }

    fn get_brightness(&mut self) -> Result<VcpValue, Error> {
        self.get_value(mccs::ImageAdjustment::Luminance.into(),
                       anyhow!("This display doesn't support brightness operations"))
    }

    fn set_brightness(&mut self, value: u16) -> Result<(), Error> {
        self.set_value(mccs::ImageAdjustment::Luminance.into(),
                       anyhow!("This display doesn't support brightness operations"),
                        value)
    }

    fn get_contrast(&mut self) -> Result<VcpValue, Error> {
        self.get_value(mccs::ImageAdjustment::Contrast.into(),
                       anyhow!("This display doesn't support contrast operations"))
    }

    fn set_contrast(&mut self, value: u16) -> Result<(), Error> {
        self.set_value(mccs::ImageAdjustment::Contrast.into(),
                       anyhow!("This display doesn't support contrast operations"),
                       value)
    }
}
