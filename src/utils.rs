use crate::data::{Continuous, Display, NonContinuous, Table, VcpReadValue, VcpWriteValue};
use ddc::{Ddc, DdcHost, DdcTable};
use ddc_hi::{Backend, FeatureCode, Query};
use mccs_db::ValueType;
use std::io::{Error, ErrorKind};

pub fn get_displays(needs_caps: bool, query: Query) -> Result<Vec<(usize, Display)>, Error> {
    Display::enumerate()
        .into_iter()
        .enumerate()
        .map(|(index, mut display)| {
            if needs_caps && display.info.backend == Backend::WinApi {
                display
                    .update_capabilities()
                    .map(|_| (index, display))
                    .map_err(|err| Error::new(ErrorKind::TimedOut, err.to_string()))
            } else {
                Ok((index, display))
            }
        })
        .filter(|display| {
            if let &Ok(ref display) = display {
                query.matches(&display.1.info)
            } else {
                true
            }
        })
        .collect()
}

pub fn get_display(needs_caps: bool, display_index: usize) -> Result<(usize, Display), Error> {
    get_displays(needs_caps, Query::Any)?
        .into_iter()
        .filter(|&(index, _)| index == display_index)
        .next()
        .ok_or(Error::new(
            ErrorKind::Unsupported,
            format!("There is no display at index: {}", display_index),
        ))
}

pub fn get_vcp_feature(
    display_index: usize,
    feature_code: FeatureCode,
) -> Result<VcpReadValue, Error> {
    let mut display = get_display(true, display_index)?.1;
    display
        .update_from_ddc()
        .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
    let feature = display.info.mccs_database.get(feature_code);
    let handle = &mut display.handle;
    if let Some(feature) = feature {
        match feature.ty {
            ValueType::Unknown => {
                let vcp_feature_value = handle
                    .get_vcp_feature(feature_code)
                    .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
                let current_value = vcp_feature_value.value();
                let maximum_value = vcp_feature_value.maximum();
                handle.sleep();
                Ok(VcpReadValue::Continuous(Continuous {
                    current_value,
                    maximum_value,
                }))
            }
            ValueType::Continuous {
                interpretation: _interpretation,
            } => {
                let vcp_feature_value = handle
                    .get_vcp_feature(feature_code)
                    .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
                let current_value = vcp_feature_value.value();
                let maximum_value = vcp_feature_value.maximum();
                handle.sleep();
                Ok(VcpReadValue::Continuous(Continuous {
                    current_value,
                    maximum_value,
                }))
            }
            ValueType::NonContinuous {
                ref values,
                interpretation: _interpretation,
            } => {
                let vcp_feature_value = handle
                    .get_vcp_feature(feature_code)
                    .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
                let mut current_value = (vcp_feature_value.value(), None);
                let possible_values = values.clone();
                if let Some(&Some(ref name)) = values.get(&(vcp_feature_value.value() as u8)) {
                    current_value.1 = Some(name.clone());
                }
                handle.sleep();
                Ok(VcpReadValue::NonContinuous(NonContinuous {
                    current_value,
                    possible_values,
                }))
            }
            ValueType::Table {
                interpretation: _interpretation,
            } => {
                let vcp_feature_value = handle
                    .table_read(feature_code)
                    .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
                handle.sleep();
                Ok(VcpReadValue::Table(Table {
                    current_data: vcp_feature_value,
                }))
            }
        }
    } else {
        match handle.table_read(feature_code) {
            Ok(vcp_feature_value) => {
                handle.sleep();
                Ok(VcpReadValue::Table(Table {
                    current_data: vcp_feature_value,
                }))
            }
            Err(_) => {
                let vcp_feature_value = handle
                    .get_vcp_feature(feature_code)
                    .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
                let current_value = vcp_feature_value.value();
                let maximum_value = vcp_feature_value.maximum();
                handle.sleep();
                Ok(VcpReadValue::Continuous(Continuous {
                    current_value,
                    maximum_value,
                }))
            }
        }
    }
}

pub fn set_vcp_feature(
    display_index: usize,
    feature_code: FeatureCode,
    value: VcpWriteValue,
) -> Result<(), Error> {
    let mut display = get_display(true, display_index)?.1;

    match value {
        VcpWriteValue::Single(value) => {
            let result = display
                .handle
                .set_vcp_feature(feature_code, value)
                .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
            display.handle.sleep();
            Ok(result)
        }
        VcpWriteValue::Bytes(bytes, offset) => {
            let result = display
                .handle
                .table_write(feature_code, offset, &bytes)
                .map_err(|error| Error::new(ErrorKind::TimedOut, error.to_string()))?;
            display.handle.sleep();
            Ok(result)
        }
    }
}
