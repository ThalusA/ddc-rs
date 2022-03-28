use ddc_hi::{Display};
use neon::prelude::*;
use crate::EnhancedDisplay;

pub fn list_infos(mut cx: FunctionContext) -> JsResult<JsArray> {
    let displays_info = Display::list_infos();
    let infos = cx.empty_array();
    let mut index = 0;
    for display_info in displays_info {
        let obj = cx.empty_object();
        let id = cx.string(display_info.id);
        obj.set(&mut cx, "id", id)?;
        if display_info.serial_number.is_some() {
            let serial_number = cx.string(display_info.serial_number.as_ref().unwrap());
            obj.set(&mut cx, "serial_number", serial_number)?;
        }
        if display_info.model_name.is_some() {
            let model_name = cx.string(display_info.model_name.as_ref().unwrap());
            obj.set(&mut cx, "model_name", model_name)?;
        }
        if (&display_info.manufacturer_id).is_some() {
            let manufacturer_id = cx.string(display_info.manufacturer_id.as_ref().unwrap());
            obj.set(&mut cx, "manufacturer_id", manufacturer_id)?;
        }
        infos.set(&mut cx, index, obj)?;
        index += 1;
    }
    infos.downcast_or_throw(&mut cx)
}

pub fn get_brightness(mut cx: FunctionContext) -> JsResult<JsArray> {
    let id: Handle<JsString> = cx.argument(0)?;
    match Display::get(id.value(&mut cx)) {
        Some(mut display) => {
            match display.get_brightness() {
                Ok(brightness) => {
                    let tuple = cx.empty_array();
                    let value = cx.number(brightness.value());
                    let maximum = cx.number(brightness.maximum());
                    tuple.set(&mut cx, 0, value)?;
                    tuple.set(&mut cx, 1, maximum)?;
                    tuple.downcast_or_throw(&mut cx)
                },
                Err(err) => cx.throw_error(err.to_string())
            }
        }
        None => {
            cx.throw_error("This display doesn't exist")
        }
    }
}

pub fn set_brightness(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id: Handle<JsString> = cx.argument(0)?;
    let value: Handle<JsNumber> = cx.argument(1)?;
    match Display::get(id.value(&mut cx)) {
        Some(mut display) => {
            match display.set_brightness(value.value(&mut cx) as u16) {
                Ok(()) => {
                    Ok(cx.undefined())
                },
                Err(err) => cx.throw_error(err.to_string())
            }
        }
        None => {
            cx.throw_error("This display doesn't exist")
        }
    }
}

pub fn get_contrast(mut cx: FunctionContext) -> JsResult<JsArray> {
    let id: Handle<JsString> = cx.argument(0)?;
    match Display::get(id.value(&mut cx)) {
        Some(mut display) => {
            match display.get_contrast() {
                Ok(contrast) => {
                    let tuple = cx.empty_array();
                    let value = cx.number(contrast.value());
                    let maximum = cx.number(contrast.maximum());
                    tuple.set(&mut cx, 0, value)?;
                    tuple.set(&mut cx, 1, maximum)?;
                    tuple.downcast_or_throw(&mut cx)
                },
                Err(err) => cx.throw_error(err.to_string())
            }
        }
        None => {
            cx.throw_error("This display doesn't exist")
        }
    }
}

pub fn set_contrast(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id: Handle<JsString> = cx.argument(0)?;
    let value: Handle<JsNumber> = cx.argument(1)?;
    match Display::get(id.value(&mut cx)) {
        Some(mut display) => {
            match display.set_contrast(value.value(&mut cx) as u16) {
                Ok(()) => {
                    Ok(cx.undefined())
                },
                Err(err) => cx.throw_error(err.to_string())
            }
        }
        None => {
            cx.throw_error("This display doesn't exist")
        }
    }
}