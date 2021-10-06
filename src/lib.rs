#[macro_use]
extern crate napi_derive;

// use napi::bindgen_prelude::*;
use napi::{
    threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunctionCallMode},
    CallContext, Env, JsFunction, JsNumber, JsObject, JsUndefined, Result as JsResult,
};
use std::{convert::TryInto, thread, time::Duration};

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
pub fn test_threadsafe_function(ctx: CallContext) -> JsResult<JsUndefined> {
    let func = ctx.get::<JsFunction>(0)?;

    let tsfn =
        ctx.env
            .create_threadsafe_function(&func, 0, |ctx: ThreadSafeCallContext<Vec<u32>>| {
                ctx.value
                    .iter()
                    .map(|v| ctx.env.create_uint32(*v))
                    .collect::<JsResult<Vec<JsNumber>>>()
            })?;

    let tsfn_cloned = tsfn.clone();

    // thread::spawn(move || {
    //     let output: Vec<u32> = vec![0, 1, 2, 3];
    //     // It's okay to call a threadsafe function multiple times.
    //     for i in 0..100 {
    //         thread::sleep(Duration::from_millis(400));
    //         tsfn.call(Ok(output.clone()), ThreadsafeFunctionCallMode::Blocking);
    //     }
    // });

    thread::spawn(move || {
        for i in 0..100 {
            let output: Vec<u32> = vec![3, 2, 1, 0];
            thread::sleep(Duration::from_millis(200));
            // It's okay to call a threadsafe function multiple times.
            tsfn_cloned.call(Ok(output.clone()), ThreadsafeFunctionCallMode::NonBlocking);
        }
    });

    ctx.env.get_undefined()
}

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> JsResult<()> {
    exports.create_named_method("fibonacci", fibonacci)?;
    exports.create_named_method("test_threadsafe_function", test_threadsafe_function)?;
    exports.set_named_property("DEFAULT_VALUE", env.create_int64(100)?)?;
    Ok(())
}
