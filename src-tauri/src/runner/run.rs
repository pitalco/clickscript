use v8::{Context, ContextScope, HandleScope, Isolate, Script, V8, new_default_platform};
use std::error::Error;

pub fn run(js_code: &str) -> Result<String, Box<dyn Error>> {
    let platform = new_default_platform(0, false).make_shared();
    V8::initialize_platform(platform);
    V8::initialize();

    let isolate = &mut Isolate::new(Default::default());

    let scope = &mut HandleScope::new(isolate);
    let context = Context::new(scope);
    let scope = &mut ContextScope::new(scope, context);

    let code = v8::String::new(scope, js_code).expect("Invalid JS code");
    let script = Script::compile(scope, code, None).expect("Failed to compile JS code");
    let result = script.run(scope).expect("Failed to run JS code");
    let result = result.to_string(scope).expect("Failed to convert result to string");
    Ok(result.to_rust_string_lossy(scope))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let js_code = "let name = \"Hello!\"; name";
        let result = run(js_code).unwrap();
        assert_eq!(result, "Hello!");
    }
}