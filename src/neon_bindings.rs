use std::cell::RefCell;
use ddc_hi::DisplayInfo;
use neon::prelude::*;
use crate::EnhancedDisplay;

type BoxedEnhancedDisplay = JsBox<RefCell<EnhancedDisplay>>;

trait StructToObject {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject>;
}

impl StructToObject for DisplayInfo {
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let id = cx.string(self.id.clone());
        obj.set(cx, "id", id)?;

        match &self.serial_number {
            Some(serial_number) => {
                let serial_number = cx.string(serial_number);
                obj.set(cx, "serial_number", serial_number)?;
            },
            None => {}
        }

        match &self.model_name {
            Some(model_name) => {
                let model_name = cx.string(model_name);
                obj.set(cx, "model_name", model_name)?;
            },
            None => {}
        }

        match &self.manufacturer_id {
            Some(manufacturer_id) => {
                let manufacturer_id = cx.string(manufacturer_id);
                obj.set(cx, "manufacturer_id", manufacturer_id)?;
            },
            None => {}
        }

        Ok(obj)
    }
}

pub fn display_new(mut cx: FunctionContext) -> JsResult<BoxedEnhancedDisplay> {
    let id = cx.argument::<JsString>(0)?.value(&mut cx);
    let display = RefCell::new(EnhancedDisplay::get(id).or_else(|error| cx.throw_error(error))?);

    Ok(cx.boxed(display))
}

pub fn display_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let displays_info = EnhancedDisplay::list_infos();
    let infos = cx.empty_array();

    for (index, display_info) in displays_info.iter().enumerate() {
        let new_object = display_info.to_object(&mut cx)?;
        infos.set(&mut cx, index as u32, new_object)?;
    }

    Ok(infos)
}

pub fn display_get_brightness(mut cx: FunctionContext) -> JsResult<JsObject> {
    let display = cx.argument::<BoxedEnhancedDisplay>(0)?;
    let mut display = display.borrow_mut();
    let obj = cx.empty_object();

    let brightness = display.get_brightness().or_else(|error| cx.throw_error(error))?;

    let brightness_value = cx.number(brightness.value());
    obj.set(&mut cx, "value", brightness_value)?;

    let brightness_maximum = cx.number(brightness.maximum());
    obj.set(&mut cx, "maximum", brightness_maximum)?;

    Ok(obj)
}


pub fn display_set_brightness(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let display = cx.argument::<BoxedEnhancedDisplay>(0)?;
    let mut display = display.borrow_mut();

    let value = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;
    display.set_brightness(value).or_else(|error| cx.throw_error(error))?;

    Ok(cx.undefined())
}

pub fn display_get_contrast(mut cx: FunctionContext) -> JsResult<JsObject> {
    let display = cx.argument::<BoxedEnhancedDisplay>(0)?;
    let mut display = display.borrow_mut();
    let obj = cx.empty_object();

    let contrast = display.get_contrast().or_else(|error| cx.throw_error(error))?;

    let contrast_value = cx.number(contrast.value());
    obj.set(&mut cx, "value", contrast_value)?;

    let contrast_maximum = cx.number(contrast.maximum());
    obj.set(&mut cx, "maximum", contrast_maximum)?;

    Ok(obj)
}


pub fn display_set_contrast(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let display = cx.argument::<BoxedEnhancedDisplay>(0)?;
    let mut display = display.borrow_mut();

    let value = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;
    display.set_contrast(value).or_else(|error| cx.throw_error(error))?;

    Ok(cx.undefined())
}
