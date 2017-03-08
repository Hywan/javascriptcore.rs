// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use super::{JSContext, JSObject, JSString, JSValue};
use sys;

/// Evaluates a string of JavaScript.
///
/// * `ctx`: The execution context to use.
/// * `script`: A string containing the script to evaluate.
/// * `this_object`: The optional object to use as `this`, or `None` to
///   use the global object as `this`.
/// * `source_url`: An optional string containing a URL for the script's
///   source file. This is used by debuggers and when reporting
///   exceptions. Pass `None` if you do not care to include source
///   file information.
/// * `starting_line_number`: An integer value specifying the script's
///   starting line number in the file located at `source_url`. This
///   is only used when reporting exceptions. The value is one-based,
///   so the first line is line `1` and invalid values are clamped
///   to `1`.
///
/// Returns either the `Option<JSValue>` that results from evaluating the script or
/// the value containing an exception that occurred.
pub fn evaluate_script<S: Into<JSString>, U: Into<JSString>>
    (ctx: &JSContext,
     script: S,
     this_object: Option<&JSObject>,
     source_url: U,
     starting_line_number: i32)
     -> Result<Option<JSValue>, JSValue> {
    unsafe {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let r = sys::JSEvaluateScript(ctx.raw,
                                      script.into().raw,
                                      this_object.map(|t| t.raw).unwrap_or(ptr::null_mut()),
                                      source_url.into().raw,
                                      starting_line_number,
                                      &mut e);
        if e.is_null() {
            Err(JSValue { raw: e })
        } else if r.is_null() {
            Ok(None)
        } else {
            Ok(Some(JSValue { raw: r }))
        }
    }
}

/// Checks for syntax errors in a string of JavaScript.
///
/// * `ctx`: The execution context to use.
/// * `script`: A string containing the script to check for
///   syntax errors.
/// * `source_url`: An optional string containing a URL for the script's
///   source file. This is only used when reporting exceptions. Pass
///   `None` if you do not care to include source file information in
///   exceptions.
/// * `starting_line_number`: An integer value specifying the script's
///   starting line number in the file located at `source_url`. This
///   is only used when reporting exceptions. The value is one-based,
///   so the first line is line `1` and invalid values are clamped
///   to `1`.
///
/// Returns `Ok` if the script is syntactically correct, otherwise
/// returns a `JSValue` containing an exception.
pub fn check_script_syntax<S: Into<JSString>, U: Into<JSString>>(ctx: &JSContext,
                                                                 script: S,
                                                                 source_url: U,
                                                                 starting_line_number: i32)
                                                                 -> Result<(), JSValue> {
    unsafe {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let r = sys::JSCheckScriptSyntax(ctx.raw,
                                         script.into().raw,
                                         source_url.into().raw,
                                         starting_line_number,
                                         &mut e);
        if r {
            Ok(())
        } else {
            Err(JSValue { raw: e })
        }
    }
}

/// Performs a JavaScript garbage collection.
///
/// JavaScript values that are on the machine stack, in a register,
/// protected by `JSValueProtect`, set as the global object of an
/// execution context, or reachable from any such value will not
/// be collected.
///
/// During JavaScript execution, you are not required to call this
/// function; the JavaScript engine will garbage collect as needed.
/// JavaScript values created within a context group are automatically
/// destroyed when the last reference to the context group is released.
///
/// * `ctx`: The execution context to use.
///
/// TODO: Fix reference to `JSValueProtect` once it has been wrapped.
pub fn garbage_collect(ctx: &JSContext) {
    unsafe {
        sys::JSGarbageCollect(ctx.raw);
    }
}

#[cfg(test)]
mod tests {
    use super::{JSContext, check_script_syntax, garbage_collect};

    #[test]
    fn can_check_script_syntax() {
        let ctx = JSContext::default();

        let r = check_script_syntax(&ctx, "alert('abc');", "test.js", 1);
        assert!(r.is_ok());

        let f = check_script_syntax(&ctx, "alert('abc", "test.js", 1);
        assert!(f.is_err());
    }

    #[test]
    fn can_garbage_collect() {
        let ctx = JSContext::default();
        garbage_collect(&ctx);
    }
}
