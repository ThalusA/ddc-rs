use neon::prelude::*;
use crate::{get_brightness, set_brightness, EnhancedDisplay,
            get_enhanced_displays, get_enhanced_display_by_id};


trait StructToObject {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>;
}

impl StructToObject for EnhancedDisplay {
    fn to_object<'a>(self: &EnhancedDisplay, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let id = cx.string(self.inner_display.info.id.clone());
        obj.set(cx, "display_id", id)?;

        let id = cx.number(self.id as f64);
        obj.set(cx, "id", id)?;

        match &self.inner_display.info.serial_number {
            Some(serial_number) => {
                let serial_number = cx.string(serial_number);
                obj.set(cx, "serial_number", serial_number)?;
            },
            None => {}
        }

        match &self.inner_display.info.model_name {
            Some(model_name) => {
                let model_name = cx.string(model_name);
                obj.set(cx, "model_name", model_name)?;
            },
            None => {}
        }

        match &self.inner_display.info.model_id {
            Some(model_id) => {
                let model_id = cx.string(model_id.to_string());
                obj.set(cx, "model_id", model_id)?;
            },
            None => {}
        }

        match &self.inner_display.info.manufacturer_id {
            Some(manufacturer_id) => {
                let manufacturer_id = cx.string(manufacturer_id);
                obj.set(cx, "manufacturer_id", manufacturer_id)?;
            },
            None => {}
        }

        Ok(obj)
    }
}

pub fn display_info(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array = cx.empty_array();

    get_enhanced_displays().into_iter().enumerate().for_each(|(index, display)| {
        let obj = display.to_object(&mut cx);
        if obj.is_ok() {
            let _ = array.set(&mut cx, index as u32, obj.unwrap());
        }
    });
    Ok(array)
}

pub fn display_get_by_id(mut cx: FunctionContext) -> JsResult<JsObject> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let display = get_enhanced_display_by_id(id)
        .or_else(|error| cx.throw_error(error.to_string()))?;
    let obj = display.to_object(&mut cx)?;
    Ok(obj)
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

