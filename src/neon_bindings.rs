use neon::prelude::*;
use crate::{ENHANCED_DISPLAYS, get_brightness, set_brightness, EnhancedDisplay};


trait StructToObject {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>;
}

impl StructToObject for EnhancedDisplay {
    fn to_object<'a>(self: &EnhancedDisplay, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let id = cx.string(self.inner_display.info.id.clone());
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
                let model_name = cx.string(model_id.to_string());
                obj.set(cx, "model_name", model_name)?;
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

    ENHANCED_DISPLAYS.with(|enhanced_displays| {
        for (index, display) in enhanced_displays.take().iter().enumerate() {
            let obj = display.to_object(&mut cx);
            match obj {
                Ok(obj) => match array.set(&mut cx, index as u32, obj) {
                    _ => ()
                }
                _ => ()
            }
        }
    });
    Ok(array)
}

pub fn display_get_brightness(mut cx: FunctionContext) -> JsResult<JsObject> {
    let display_id = cx.argument::<JsString>(0)?.value(&mut cx);
    let obj = cx.empty_object();

    let brightness = get_brightness(display_id)
        .or_else(|error| cx.throw_error(error.to_string()))?;

    let brightness_value = cx.number(brightness.value());
    obj.set(&mut cx, "value", brightness_value)?;

    let brightness_maximum = cx.number(brightness.maximum());
    obj.set(&mut cx, "maximum", brightness_maximum)?;

    Ok(obj)
}


pub fn display_set_brightness(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let display_id = cx.argument::<JsString>(0)?.value(&mut cx);
    let value = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;

    set_brightness(display_id, value)
        .or_else(|error| cx.throw_error(error.to_string()))?;

    Ok(cx.undefined())
}

