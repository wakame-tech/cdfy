use anyhow::Result;
use cdfy_runtime::spec::bindings::Runtime;
use std::{fs::File, io::Read};

pub mod runner;

pub struct WasmPlugin {
    pub runtime: Runtime,
}

impl Default for WasmPlugin {
    fn default() -> Self {
        let mut wasm = vec![];
        File::open("../../../../.cache/counter_server.wasm")
            .unwrap()
            .read_to_end(&mut wasm)
            .unwrap();
        WasmPlugin::new(&wasm).unwrap()
    }
}

impl WasmPlugin {
    pub fn new(wasm: &[u8]) -> Result<Self> {
        Ok(Self {
            runtime: Runtime::new(wasm)?,
        })
    }
}
