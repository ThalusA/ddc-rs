use neon::prelude::*;

pub fn byte_vec_to_bytearray<'a, C: Context<'a>>(
    vec: &Vec<u8>,
    cx: &mut C,
) -> JsResult<'a, JsArrayBuffer> {
    let array = cx.array_buffer(vec.len())?;

    for (index, element) in vec.clone().into_iter().enumerate() {
        let element = cx.number(element);
        array.set(cx, index as u32, element)?;
    }
    Ok(array)
}
