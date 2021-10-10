#[macro_use]
extern crate napi_derive;

// use napi::bindgen_prelude::*;
use napi::{
    CallContext, Env, JsNumber, JsObject, JsString, 
    Result as JsResult,
};
use std::{convert::TryInto};

#[js_function(1)] // ------> arguments length
fn fibonacci(ctx: CallContext) -> JsResult<JsNumber> {
    let n = ctx.get::<JsNumber>(0)?.try_into()?;
    ctx.env.create_int64(fibonacci_native(n))
}

#[inline(always)]
fn fibonacci_native(n: i64) -> i64 {
    match n {
        1 | 2 => 1,
        _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
    }
}

#[js_function(1)]
pub fn hello(ctx: CallContext) -> JsResult<JsString> {
    let input = ctx.get::<JsString>(0)?;
    let input = input.into_utf16()?.as_str()?;

    let mut output = String::from("hello ");
    output.push_str(&input);

    ctx.env.create_string(&output)
}

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> JsResult<()> {
    exports.create_named_method("fibonacci", fibonacci)?;
    exports.create_named_method("hello", hello)?;
    exports.set_named_property("DEFAULT_VALUE", env.create_int64(100)?)?;
    Ok(())
}
