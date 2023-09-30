#![allow(unused)]
use super::types::*;
use fp_bindgen_support::{
    common::{abi::WasmAbi, mem::FatPtr},
    wasmer2_host::{
        errors::{InvocationError, RuntimeError},
        mem::{
            deserialize_from_slice, export_to_guest, export_to_guest_raw, import_from_guest,
            import_from_guest_raw, serialize_to_vec,
        },
        r#async::{create_future_value, future::ModuleRawFuture, resolve_async_value},
        runtime::RuntimeInstanceData,
    },
};
use std::cell::RefCell;
use wasmer::{imports, Function, ImportObject, Instance, Module, Store, WasmerEnv};

#[derive(Clone)]
pub struct Runtime {
    instance: Instance,
    env: RuntimeInstanceData,
}

impl Runtime {
    pub fn new(wasm_module: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let store = Self::default_store();
        let module = Module::new(&store, wasm_module)?;
        let mut env = RuntimeInstanceData::default();
        let import_object = create_import_object(module.store(), &env);
        let instance = Instance::new(&module, &import_object).unwrap();
        env.init_with_instance(&instance).unwrap();
        Ok(Self { instance, env })
    }

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    fn default_store() -> wasmer::Store {
        let compiler = wasmer::Cranelift::default();
        let engine = wasmer::Universal::new(compiler).engine();
        Store::new(&engine)
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    fn default_store() -> wasmer::Store {
        let compiler = wasmer::Singlepass::default();
        let engine = wasmer::Universal::new(compiler).engine();
        Store::new(&engine)
    }

    pub fn default_state(&self) -> Result<IResult, InvocationError> {
        let result = self.default_state_raw();
        let result = result.map(|ref data| deserialize_from_slice(data));
        result
    }
    pub fn default_state_raw(&self) -> Result<Vec<u8>, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<(), FatPtr>("__fp_gen_default_state")
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_default_state".to_owned())
            })?;
        let result = function.call()?;
        let result = import_from_guest_raw(&self.env, result);
        Ok(result)
    }

    pub fn on_event(&self, state: String, event: String) -> Result<IResult, InvocationError> {
        let state = serialize_to_vec(&state);
        let event = serialize_to_vec(&event);
        let result = self.on_event_raw(state, event);
        let result = result.map(|ref data| deserialize_from_slice(data));
        result
    }
    pub fn on_event_raw(&self, state: Vec<u8>, event: Vec<u8>) -> Result<Vec<u8>, InvocationError> {
        let state = export_to_guest_raw(&self.env, state);
        let event = export_to_guest_raw(&self.env, event);
        let function = self
            .instance
            .exports
            .get_native_function::<(FatPtr, FatPtr), FatPtr>("__fp_gen_on_event")
            .map_err(|_| InvocationError::FunctionNotExported("__fp_gen_on_event".to_owned()))?;
        let result = function.call(state.to_abi(), event.to_abi())?;
        let result = import_from_guest_raw(&self.env, result);
        Ok(result)
    }

    pub fn plugin_meta(&self) -> Result<PluginMeta, InvocationError> {
        let result = self.plugin_meta_raw();
        let result = result.map(|ref data| deserialize_from_slice(data));
        result
    }
    pub fn plugin_meta_raw(&self) -> Result<Vec<u8>, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<(), FatPtr>("__fp_gen_plugin_meta")
            .map_err(|_| InvocationError::FunctionNotExported("__fp_gen_plugin_meta".to_owned()))?;
        let result = function.call()?;
        let result = import_from_guest_raw(&self.env, result);
        Ok(result)
    }
}

fn create_import_object(store: &Store, env: &RuntimeInstanceData) -> ImportObject {
    imports! {
        "fp" => {
            "__fp_host_resolve_async_value" => Function::new_native_with_env(store, env.clone(), resolve_async_value),
            "__fp_gen_debug" => Function::new_native_with_env(store, env.clone(), _debug),
            "__fp_gen_rand" => Function::new_native_with_env(store, env.clone(), _rand),
        }
    }
}

pub fn _debug(env: &RuntimeInstanceData, message: FatPtr) {
    let message = import_from_guest::<String>(env, message);
    super::debug(message)
}

pub fn _rand(env: &RuntimeInstanceData) -> <u32 as WasmAbi>::AbiType {
    super::rand().to_abi()
}
