#[macro_use]
extern crate napi_derive;

// use napi::bindgen_prelude::*;
use napi::{CallContext, Env, JsNumber, JsObject, Result as JsResult};

use std::convert::TryInto;


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

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> JsResult<()> {
    exports.create_named_method("fibonacci", fibonacci)?;
    exports.set_named_property("DEFAULT_VALUE", env.create_int64(100)?)?;
    Ok(())
}
