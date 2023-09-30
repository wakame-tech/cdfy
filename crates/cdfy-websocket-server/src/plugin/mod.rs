use anyhow::Result;
use cdfy_runtime::spec::bindings::Runtime;

pub mod runner;

pub struct WasmPlugin {
    pub runtime: Runtime,
}

impl WasmPlugin {
    pub fn new(wasm: &[u8]) -> Result<Self> {
        Ok(Self {
            runtime: Runtime::new(wasm)?,
        })
    }
}
