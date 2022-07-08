use crate::neon_utils::byte_vec_to_bytearray;
use ddc_hi::Backend;
use neon::prelude::*;
use std::collections::BTreeMap;
use std::str::FromStr;

pub trait StructToObject {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>;
}

pub trait StructFromObject<T> {
    fn from_object<'a>(cx: &mut impl Context<'a>, object: Handle<'a, JsObject>) -> Option<T>;
}

pub use ddc_hi::Display;

impl StructToObject for Display {
    fn to_object<'a>(self: &Display, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let struct_object = cx.empty_object();

        let display_id = cx.string(self.info.id.clone());
        struct_object.set(cx, "displayId", display_id)?;

        if let &Some(serial) = &self.info.serial {
            let serial = cx.number(serial as f64);
            struct_object.set(cx, "serial", serial)?;
        }

        if let Some(serial_number) = &self.info.serial_number {
            let serial_number = cx.string(serial_number);
            struct_object.set(cx, "serialNumber", serial_number)?;
        }

        if let Some(model_name) = &self.info.model_name {
            let model_name = cx.string(model_name);
            struct_object.set(cx, "modelName", model_name)?;
        }

        if let &Some(model_id) = &self.info.model_id {
            let model_id = cx.string(model_id.to_string());
            struct_object.set(cx, "modelId", model_id)?;
        }

        if let &Some((major, minor)) = &self.info.version {
            let version = cx.string(format!("{}.{}", major, minor));
            struct_object.set(cx, "version", version)?;
        }

        if let Some(manufacturer_id) = &self.info.manufacturer_id {
            let manufacturer_id = cx.string(manufacturer_id);
            struct_object.set(cx, "manufacturerId", manufacturer_id)?;
        }

        if let &Some(manufacture_week) = &self.info.manufacture_week {
            let manufacture_week = cx.number(manufacture_week as f64);
            struct_object.set(cx, "manufactureWeek", manufacture_week)?;
        }

        if let &Some(manufacture_year) = &self.info.manufacture_year {
            let manufacture_year = cx.number(manufacture_year as f64);
            struct_object.set(cx, "manufactureYear", manufacture_year)?;
        }

        if let &Some(mccs_version) = &self.info.mccs_version {
            let mccs_version = cx.string(mccs_version.to_string());
            struct_object.set(cx, "mccsVersion", mccs_version)?;
        }

        if let Some(edid_data) = &self.info.edid_data {
            let edid_data = byte_vec_to_bytearray(edid_data, cx)?;
            struct_object.set(cx, "edidData", edid_data)?;
        }

        let backend = cx.string(&self.info.backend.to_string());
        struct_object.set(cx, "backend", backend)?;

        Ok(struct_object)
    }
}

pub use ddc_hi::Query;

enum QueryType {
    Backend = 0,
    Id = 1,
    ManufacturerId = 2,
    ModelName = 3,
    SerialNumber = 4,
}

impl QueryType {
    fn from_usize(value: usize) -> Option<QueryType> {
        match value {
            0 => Some(QueryType::Backend),
            1 => Some(QueryType::Id),
            2 => Some(QueryType::ManufacturerId),
            3 => Some(QueryType::ModelName),
            4 => Some(QueryType::SerialNumber),
            _ => None,
        }
    }
}

impl StructFromObject<Query> for Query {
    fn from_object<'a>(cx: &mut impl Context<'a>, object: Handle<'a, JsObject>) -> Option<Query> {
        let query_type = object.get::<JsNumber, _, _>(cx, "queryType");
        let query_value = object.get::<JsString, _, _>(cx, "queryValue");
        match (query_type, query_value) {
            (Ok(query_type_handle), Ok(query_value_handle)) => {
                let query_type = QueryType::from_usize(query_type_handle.value(cx) as usize);
                let query_value = query_value_handle.value(cx);
                match query_type {
                    Some(QueryType::Backend) => Backend::from_str(query_value.as_str())
                        .map(|backend| Some(Query::Backend(backend)))
                        .unwrap_or(None),
                    Some(QueryType::Id) => Some(Query::Id(query_value)),
                    Some(QueryType::ManufacturerId) => Some(Query::ManufacturerId(query_value)),
                    Some(QueryType::ModelName) => Some(Query::ModelName(query_value)),
                    Some(QueryType::SerialNumber) => Some(Query::SerialNumber(query_value)),
                    None => None,
                }
            }
            _ => None,
        }
    }
}

pub struct Continuous {
    pub current_value: u16,
    pub maximum_value: u16,
}

impl StructToObject for Continuous {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let struct_object = cx.empty_object();

        let current_value = cx.number(self.current_value);
        struct_object.set(cx, "currentValue", current_value)?;

        let maximum_value = cx.number(self.maximum_value);
        struct_object.set(cx, "maximumValue", maximum_value)?;

        let struct_type = cx.string("CONTINUOUS");
        struct_object.set(cx, "type", struct_type)?;

        Ok(struct_object)
    }
}

pub struct NonContinuous {
    pub current_value: (u16, Option<String>),
    pub possible_values: BTreeMap<u8, Option<String>>,
}

impl StructToObject for NonContinuous {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let struct_object = cx.empty_object();

        let current_value = cx.empty_array();
        let current_value_number = cx.number(self.current_value.0);
        current_value.set(cx, 0, current_value_number)?;
        if let Some(current_value_string) = &self.current_value.1 {
            let current_value_string = cx.string(current_value_string);
            current_value.set(cx, 1, current_value_string)?;
        }
        struct_object.set(cx, "currentValue", current_value)?;

        let possible_values = cx.empty_object();
        for (&number, string) in self.possible_values.iter() {
            let possible_value_number = cx.number(number);
            if let Some(possible_value_string) = string {
                let possible_value_string = cx.string(possible_value_string);
                possible_values.set(cx, possible_value_number, possible_value_string)?;
            } else {
                let undefined = cx.undefined();
                possible_values.set(cx, possible_value_number, undefined)?;
            }
        }

        let struct_type = cx.string("NON_CONTINUOUS");
        struct_object.set(cx, "type", struct_type)?;

        Ok(struct_object)
    }
}

pub struct Table {
    pub current_data: Vec<u8>,
}

impl StructToObject for Table {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let struct_object = cx.empty_object();

        let current_data = &self.current_data;
        let current_data = byte_vec_to_bytearray(current_data, cx)?;
        struct_object.set(cx, "currentData", current_data)?;

        let struct_type = cx.string("TABLE");
        struct_object.set(cx, "type", struct_type)?;

        Ok(struct_object)
    }
}

pub enum VcpReadValue {
    Continuous(Continuous),
    NonContinuous(NonContinuous),
    Table(Table),
}

impl StructToObject for VcpReadValue {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        match self {
            VcpReadValue::Continuous(continuous) => continuous.to_object(cx),
            VcpReadValue::NonContinuous(non_continuous) => non_continuous.to_object(cx),
            VcpReadValue::Table(table) => table.to_object(cx),
        }
    }
}

pub enum VcpWriteValue {
    Single(u16),
    Bytes(Vec<u8>, u16),
}
