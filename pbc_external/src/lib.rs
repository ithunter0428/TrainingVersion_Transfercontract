#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "ext"))]
extern "C" {
    pub fn read_context_into_address(addr: i64, len: i32) -> i32;
    pub fn call_named(name_ptr: i64, name_len: i32, rpc_ptr: i64, rpc_len: i32);
    pub fn log_external(message_ptr: i64, message_len: i32);
}

#[cfg(not(target_arch = "wasm32"))]
mod dummy {
    #[no_mangle]
    pub extern "C" fn read_context_into_address(_addr: i64, _len: i32) -> i32 {
        0
    }

    #[no_mangle]
    pub extern "C" fn call_named(_name_ptr: i64, _name_len: i32, _rpc_ptr: i64, _rpc_len: i32) {}

    #[no_mangle]
    pub extern "C" fn log_external(_message_ptr: i64, _message_len: i32) {}
}
