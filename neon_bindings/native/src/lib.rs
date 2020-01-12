#[macro_use]
extern crate neon;
extern crate rust_electron;

use neon::prelude::*;

fn hello_world(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(rust_electron::hello_world()))
}

/// Converts image::RgbaImage into a JsArrayBuffer
fn render_image(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let image = rust_electron::render_image();
    // each pixel holds 4 bytes of information (RGBA)
    let mut array_buffer = cx.array_buffer(image.width() * image.height() * 4)?;
    cx.borrow_mut(&mut array_buffer, |data| {
        let slice = data.as_mut_slice::<u8>();
        for (i, pixel) in image.pixels().enumerate() {
            slice[i*4 + 0] = pixel[0];
            slice[i*4 + 1] = pixel[1];
            slice[i*4 + 2] = pixel[2];
            slice[i*4 + 3] = pixel[3];
        }
    });

    Ok(array_buffer)
}

register_module!(mut cx, {
    cx.export_function("hello_world", hello_world)?;
    cx.export_function("render_image", render_image)
});
