use anyhow::Result;
use regex;
use std::path::Path;

use wasmtime::{Config, Engine, Instance, Linker, Module, Store};
use wasmtime_wasi;
use wit_bindgen_wasmtime;

fn default_config() -> Result<Config> {
    let mut config = Config::new();
    config.debug_info(true);
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    Ok(config)
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}

struct Context<E> {
    wasi: wasmtime_wasi::WasiCtx,
    exports: E,
}

fn instantiate<E: Default, T>(
    wasm: &str,
    mk_exports: impl FnOnce(
        &mut Store<Context<E>>,
        &Module,
        &mut Linker<Context<E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<E>>)> {
    let engine = Engine::new(&default_config()?)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<E>| &mut cx.wasi)?;

    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            exports: E::default(),
        },
    );
    let (exports, _instance) = mk_exports(&mut store, &module, &mut linker)?;
    Ok((exports, store))
}

wit_bindgen_wasmtime::import!("debugger.wit");

fn run(wasm: &str) -> Result<()> {
    let (exports, mut store) = crate::instantiate(wasm, |store, module, linker| {
        debugger::Debugger::instantiate(store, module, linker, |cx| &mut cx.exports)
    })?;

    exports.handle_json(&mut store, "hello".into(), "bob".as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // print usage if no args
    if args.len() < 2 {
        println!("Usage: {} <path/to/project>", args[0]);
        std::process::exit(1);
    }

    let target_path = args[1].clone();
    let target_package_regex = regex::Regex::new(r"examples/rust/([^/]+)").unwrap();
    let target_package = target_package_regex
        .captures(&target_path)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or(format!(
            "Could not find wasm rust package at path {}",
            target_path
        ))?;

    let target_wasm_path = format!(
        "target/wasm32-unknown-unknown/debug/{}.wasm",
        target_package
    );
    let target_wit_path = format!("examples/rust/{}/{}.wit", target_package, target_package);

    if !Path::new(&target_wasm_path).exists() {
        return Err(format!("Could not find wasm file at path {}", target_wasm_path).into());
    }
    let wit_exists = Path::new(&target_wit_path).exists();

    println!("debugging: {}", target_wasm_path);
    if wit_exists {
        println!("with wit: {}", target_wit_path);
    }

    run(&target_wasm_path)?;

    Ok(())
}
