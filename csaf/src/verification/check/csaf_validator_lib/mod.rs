use std::borrow::Cow;
use std::env;
use std::rc::Rc;
use deno_core::{include_js_files, ExtensionBuilder, ExtensionFileSource, JsRuntime, RuntimeOptions, Extension, ModuleCodeString};
use deno_core::error::AnyError;

fn create_runtime() -> JsRuntime {
    let csaf_validator_lib = ExtensionBuilder::default()
        .js(include_js_files!(csaf_validator_lib dir "src/verification/check/csaf_validator_lib/js", "bundle.js",).into())
        .build();

    let runtime = JsRuntime::new(RuntimeOptions {
        // extensions: vec![csaf_validator_lib],
        ..Default::default()
    });

    runtime
}
async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    let  a = js_runtime.execute_script("[runjs:bundle.js]",  deno_core::FastString::Static("js/bundle.js")).unwrap();
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(deno_core::PollEventLoopOptions::default()).await?;
    result.await?;
    {
        let scope = &mut js_runtime.handle_scope();
        let value = a.open(scope);
        println!("Result from JavaScript: {:?}", value.to_string(scope).unwrap().to_rust_string_lossy(scope));
    }
    Ok(())
}
#[cfg(test)]
// #[cfg(feature = "csaf-validator-lib")]
mod test {
    use super::*;
    use tokio::runtime::Handle;


    #[tokio::test]
    async fn test_return_value() {
        let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            extensions: vec![],
            ..Default::default()
        });

        js_runtime.execute_script("add.js",deno_core::FastString::Static(include_str!("add.js")));

        let result = js_runtime.execute_script(
            "test.js",
            deno_core::FastString::Static("add(5, 3);"),
        ).unwrap();

        {
            let scope = &mut js_runtime.handle_scope();
            let value = result.open(scope);
            println!("Result from JavaScript: {:?}", value.to_string(scope).unwrap().to_rust_string_lossy(scope));
        }
    }

    #[tokio::test]
    // #[cfg(feature = "csaf-validator-lib")]
    async fn test() -> anyhow::Result<()>{

        let mut runtime = create_runtime();
        let result = runtime.execute_script_static("bundle.js", include_str!("js/bundle.js"));
        let result =
            runtime.execute_script_static("main.js", include_str!("js/main.js")).unwrap();
            {
                let scope = &mut runtime.handle_scope();
                let value = result.open(scope);
                println!("Result from JavaScript: {:?}", value.to_string(scope).unwrap().to_rust_string_lossy(scope));
            }
        Ok(())
        // aaa();
        // result.unwrap().unwrap();
        // assert!(false);
    }
}
