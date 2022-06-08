use anyhow::Result;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi;
use wit_bindgen_wasmtime;

wit_bindgen_wasmtime::import!("debugger.wit");

struct Context {
    wasi: wasmtime_wasi::WasiCtx,
    debugger_state: debugger::DebuggerData,
}

#[derive(Clone)]
pub struct HandleFactory {
    engine: Engine,
    linker: Linker<Context>,
    module: Module,
}

impl HandleFactory {
    fn default_config() -> Result<Config> {
        let mut config = Config::new();
        config.debug_info(true);
        config.cache_config_load_default()?;
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        Ok(config)
    }

    fn default_wasi() -> wasmtime_wasi::WasiCtx {
        wasmtime_wasi::sync::WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build()
    }

    pub fn new(wasm_path: &str) -> Result<Self> {
        let engine = Engine::new(&Self::default_config()?)?;
        let module = Module::from_file(&engine, wasm_path)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context| &mut cx.wasi)?;
        debugger::Debugger::add_to_linker(&mut linker, |cx: &mut Context| &mut cx.debugger_state)?;

        Ok(Self {
            engine,
            linker,
            module,
        })
    }

    pub fn make_handler(&self) -> Result<Handler> {
        let mut store = Store::new(
            &self.engine,
            Context {
                wasi: Self::default_wasi(),
                debugger_state: debugger::DebuggerData::default(),
            },
        );
        let linked = self.linker.instantiate(&mut store, &self.module)?;
        let instance = debugger::Debugger::new(&mut store, &linked, |cx: &mut Context| {
            &mut cx.debugger_state
        })?;

        Ok(Handler { store, instance })
    }
}

pub struct Handler {
    store: Store<Context>,
    instance: debugger::Debugger<Context>,
}

impl Handler {
    pub fn handle_json(&mut self, name: String, json: Vec<u8>) -> Result<Vec<u8>> {
        match self.instance.handle_json(&mut self.store, &name, &json) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.into()),
        }
    }
}
