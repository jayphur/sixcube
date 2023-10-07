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

    pub fn test_host_fn(&self) -> Result<u32, InvocationError> {
        let result = self.test_host_fn_raw();
        result
    }
    pub fn test_host_fn_raw(&self) -> Result<u32, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<(), <u32 as WasmAbi>::AbiType>("__fp_gen_test_host_fn")
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_test_host_fn".to_owned())
            })?;
        let result = function.call()?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }
}

fn create_import_object(store: &Store, env: &RuntimeInstanceData) -> ImportObject {
    imports! {
        "fp" => {
            "__fp_host_resolve_async_value" => Function::new_native_with_env(store, env.clone(), resolve_async_value),
            "__fp_gen_test_plugin_fn" => Function::new_native_with_env(store, env.clone(), _test_plugin_fn),
        }
    }
}

pub fn _test_plugin_fn(env: &RuntimeInstanceData) -> <u32 as WasmAbi>::AbiType {
    super::test_plugin_fn().to_abi()
}
