#[macro_use]
extern crate neon;
extern crate rust_electron;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(rust_electron::hello_world()))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
