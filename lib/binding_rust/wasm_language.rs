use crate::{ffi, Language, LanguageError, Parser};
use std::{ffi::CString, mem, os::raw::c_char};
pub use wasmtime;

#[cfg(feature = "wasm")]
pub fn test() {
    wasmtime_c_api::wasm_engine_new();
}

#[repr(C)]
#[derive(Clone)]
pub struct wasm_engine_t {
    pub(crate) engine: wasmtime::Engine,
}

pub struct WasmStore(*mut ffi::TSWasmStore);

impl WasmStore {
    pub fn new(engine: wasmtime::Engine) -> Self {
        let mut c_engine = Box::new(wasm_engine_t {
            engine: engine.clone(),
        });
        let result = WasmStore(unsafe {
            ffi::ts_wasm_store_new(c_engine.as_mut() as *mut wasm_engine_t as *mut _)
        });
        mem::forget(c_engine);
        result
    }

    pub fn load_language(&mut self, name: &str, bytes: &[u8]) -> Language {
        let name = CString::new(name).unwrap();
        Language(unsafe {
            ffi::ts_wasm_store_load_language(
                self.0,
                name.as_ptr(),
                bytes.as_ptr() as *const c_char,
                bytes.len() as u32,
            )
        })
    }
}

impl Language {
    pub fn is_wasm(&self) -> bool {
        unsafe { ffi::ts_language_is_wasm(self.0) }
    }
}

impl Parser {
    pub fn set_wasm_store(&mut self, store: WasmStore) -> Result<(), LanguageError> {
        unsafe {
            ffi::ts_parser_set_wasm_store(self.0.as_ptr(), store.0);
        }
        mem::forget(store);
        Ok(())
    }
}

impl Drop for WasmStore {
    fn drop(&mut self) {
        unsafe { ffi::ts_wasm_store_delete(self.0) };
    }
}
