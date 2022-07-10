extern crate core;

mod data;
mod neon_utils;
mod utils;

use crate::data::{StructFromObject, StructToObject, StructToObjectMut, VcpWriteValue};
use ddc_hi::{FeatureCode, Query};
use neon::prelude::*;
use neon::types::buffer::TypedArray;

pub fn display_get_vcp_feature(mut cx: FunctionContext) -> JsResult<JsObject> {
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let feature_code = cx.argument::<JsNumber>(1)?.value(&mut cx) as FeatureCode;

    match utils::get_vcp_feature(index, feature_code) {
        Ok(value) => value.to_object(&mut cx),
        Err(error) => cx.throw_error(error.to_string())?,
    }
}

pub fn display_set_vcp_feature(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let feature_code = cx.argument::<JsNumber>(1)?.value(&mut cx) as FeatureCode;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as u16;

    let vcp_write_value = VcpWriteValue::Single(value);

    match utils::set_vcp_feature(index, feature_code, vcp_write_value) {
        Ok(_) => Ok(cx.undefined()),
        Err(error) => cx.throw_error(error.to_string())?,
    }
}

pub fn display_set_table_vcp_feature(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let feature_code = cx.argument::<JsNumber>(1)?.value(&mut cx) as FeatureCode;
    let offset = cx.argument::<JsNumber>(3)?.value(&mut cx) as u16;
    let data = cx.argument::<JsArrayBuffer>(2)?;
    let data = data.as_slice(&mut cx);

    let vcp_write_value = VcpWriteValue::Bytes(Vec::from(data), offset);

    match utils::set_vcp_feature(index, feature_code, vcp_write_value) {
        Ok(_) => Ok(cx.undefined()),
        Err(error) => cx.throw_error(error.to_string())?,
    }
}

pub fn display_manager_get_by_index(mut cx: FunctionContext) -> JsResult<JsObject> {
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;

    let (index, mut display) =
        utils::get_display(false, index).or_else(|error| cx.throw_error(error.to_string()))?;
    let display_object = display.to_object_mut(&mut cx)?;
    let index = cx.number(index as f64);
    display_object.set(&mut cx, "index", index)?;

    Ok(display_object)
}

pub fn display_manager_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let queries = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
    let mut effective_queries: Vec<Query> = vec![];

    for query in queries {
        if query.is_a::<JsObject, _>(&mut cx) {
            let query = query.downcast::<JsObject, _>(&mut cx).unwrap();
            if let Some(query) = Query::from_object(&mut cx, query) {
                effective_queries.push(query);
            }
        }
    }
    let final_query = if effective_queries.len() == 0 {
        Query::Any
    } else {
        Query::And(effective_queries)
    };

    let displays = utils::get_displays(false, final_query)
        .or_else(|error| cx.throw_error(error.to_string()))?;
    let displays_array = cx.empty_array();
    for (array_index, (index, mut display)) in displays.into_iter().enumerate() {
        let display_object = display.to_object_mut(&mut cx)?;
        let index = cx.number(index as f64);
        display_object.set(&mut cx, "index", index)?;
        displays_array.set(&mut cx, array_index as u32, display_object)?;
    }

    Ok(displays_array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("displayGetVcpFeature", display_get_vcp_feature)?;
    cx.export_function("displaySetVcpFeature", display_set_vcp_feature)?;
    cx.export_function("displaySetTableVcpFeature", display_set_table_vcp_feature)?;
    cx.export_function("displayManagerGetByIndex", display_manager_get_by_index)?;
    cx.export_function("displayManagerList", display_manager_list)?;
    Ok(())
}
