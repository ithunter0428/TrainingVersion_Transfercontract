extern crate pbc_external;

use std::io::Cursor;

use pbc_external::*;

use crate::serialization::ReadInt;

pub fn raw_log(message: &str) {
    let string = message.to_string();
    let len = string.len();
    unsafe {
        log_external(string.as_ptr() as i64, len as i32);
    }
}

pub fn info(string: String) {
    raw_log(&string);
}

unsafe fn read_rpc() -> Cursor<Vec<u8>> {
    let mut buf = [0u8; 65536];
    let buf_ptr = buf.as_mut_ptr() as i64;

    let len = read_context_into_address(buf_ptr, buf.len() as i32) as usize;

    let mut result: Vec<u8> = Vec::with_capacity(buf.len());
    result.extend_from_slice(&buf);
    result.resize(len, 0);

    Cursor::new(result)
}

fn remove_function_name_hash(mut vec: Vec<u8>) -> Vec<u8> {
    vec.split_off(4)
}

unsafe fn dispatch(name: &str, cursor: Cursor<Vec<u8>>) {
    let len = name.len();
    let inner = cursor.into_inner();
    let mut vec = remove_function_name_hash(inner);
    let vec_len = vec.len() as i32;

    info(format!("vec len: {}", vec_len));
    // these pointers are guaranteed to be valid since the content they point to wont be dropped
    // until we're done. (In particular, `call_named` does not save these pointers somewhere.)
    call_named(
        name.as_ptr() as i64,
        len as i32,
        vec.as_mut_ptr() as i64,
        vec_len,
    );
}

fn create_action_name(hash_as_int: i32) -> String {
    format!("action_{:x}", hash_as_int)
}

#[no_mangle]
pub unsafe extern "C" fn raw_execute() {
    let mut rpc_buf = read_rpc();
    let action_name = create_action_name(rpc_buf.read_i32_be());
    dispatch(&action_name, rpc_buf);
}
