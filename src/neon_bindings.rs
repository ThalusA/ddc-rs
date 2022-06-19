use ddc_hi::Display;
use neon::prelude::*;
use crate::{get_brightness, set_brightness, get_displays, does_display_support_ddc};


trait StructToObject {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>;
}

impl StructToObject for Display {
    fn to_object<'a>(self: &Display, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let display_id = cx.string(self.info.id.clone());
        obj.set(cx, "display_id", display_id)?;

        match &self.info.serial_number {
            Some(serial_number) => {
                let serial_number = cx.string(serial_number);
                obj.set(cx, "serial_number", serial_number)?;
            }
            None => {}
        }

        match &self.info.model_name {
            Some(model_name) => {
                let model_name = cx.string(model_name);
                obj.set(cx, "model_name", model_name)?;
            }
            None => {}
        }

        match &self.info.model_id {
            Some(model_id) => {
                let model_id = cx.string(model_id.to_string());
                obj.set(cx, "model_id", model_id)?;
            }
            None => {}
        }

        match &self.info.manufacturer_id {
            Some(manufacturer_id) => {
                let manufacturer_id = cx.string(manufacturer_id);
                obj.set(cx, "manufacturer_id", manufacturer_id)?;
            }
            None => {}
        }

        Ok(obj)
    }
}

pub fn displays_info(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array = cx.empty_array();

    get_displays(false)
        .or_else(|error| cx.throw_error(error.to_string()))?
        .iter().enumerate()
        .for_each(|(id, display)| {
            let obj = display.to_object(&mut cx);
            if obj.is_ok() {
                let obj = obj.unwrap();
                let string_id = cx.string(id.to_string());
                let _ = obj.set(&mut cx, "id", string_id);
                let _ = array.set(&mut cx, id as u32, obj);
            }
    });

    Ok(array)
}

pub fn display_support_ddc(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;

    let display_does_support_ddc = does_display_support_ddc(id)
        .or_else(|error| cx.throw_error(error.to_string()))?;

    Ok(cx.boolean(display_does_support_ddc))
}

pub fn display_get_brightness(mut cx: FunctionContext) -> JsResult<JsObject> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let obj = cx.empty_object();

    let brightness = get_brightness(id)
        .or_else(|error| cx.throw_error(error.to_string()))?;

    let brightness_value = cx.number(brightness.value());
    obj.set(&mut cx, "value", brightness_value)?;

    let brightness_maximum = cx.number(brightness.maximum());
    obj.set(&mut cx, "maximum", brightness_maximum)?;

    Ok(obj)
}


pub fn display_set_brightness(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let value = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;

    set_brightness(id, value)
        .or_else(|error| cx.throw_error(error.to_string()))?;

    Ok(cx.undefined())
}

