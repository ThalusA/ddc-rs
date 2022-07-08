use neon::prelude::*;

pub fn byte_vec_to_bytearray<'a, C: Context<'a>>(
    vec: &Vec<u8>,
    cx: &mut C,
) -> JsResult<'a, JsArrayBuffer> {
    let array: [u8; 256] = vec
        .clone()
        .try_into()
        .or_else(|_| cx.throw_error("vector data type doesn't fit in a [u8;256]"))?;
    let array = JsArrayBuffer::external(cx, array);
    Ok(array)
}
