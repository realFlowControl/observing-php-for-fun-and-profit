use std::fs::OpenOptions;

use ext_php_rs::{ffi::{_zend_file_handle, _zend_op_array, _zend_string, zend_compile_file}, prelude::*};
use structured_logger::{json::new_writer, Builder};

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

extern "C" fn startup(_type: i32, _num: i32) -> i32 {
    println!("STARTUP");
    unsafe {
        PREV_ZEND_COMPILE_FILE = zend_compile_file;
        zend_compile_file = Some(compile_file);
    }
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/logs/log.json")
        .expect("Failed to open log file");
    Builder::with_level("info")
        .with_target_writer("*", new_writer(log_file))
        .init();
    0
}

extern "C" fn shutdown(_type: i32, _num: i32) -> i32 {
    println!("SHUTDOWN");
    0
}

extern "C" fn rinit(_type: i32, _num: i32) -> i32 {
    println!("RINIT");
    0
}

extern "C" fn rshutdown(_type: i32, _num: i32) -> i32 {
    println!("RSHUTDOWN");
    0
}

static mut PREV_ZEND_COMPILE_FILE: Option<unsafe extern "C" fn(*mut _zend_file_handle, i32) -> *mut _zend_op_array> = None;

unsafe extern "C" fn compile_file (file: *mut _zend_file_handle, r#type: i32) -> *mut _zend_op_array {
    unsafe {
        if let Some(fun) = PREV_ZEND_COMPILE_FILE {
            let start = std::time::Instant::now();
            let op_array = fun(file, r#type);
            let duration = start.elapsed();
            log::info!(duration = duration.as_nanos(); "File compiled");
            println!("duration: {}", duration.as_nanos());
            return op_array;
        }
    }
    std::ptr::null_mut()
}

#[php_module]
#[php(startup=startup)]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(hello_world))
        .startup_function(startup)
        .shutdown_function(shutdown)
        .request_startup_function(rinit)
        .request_shutdown_function(rshutdown)
}

#[allow(unused)]
unsafe fn zend_string_to_string(zs: *const _zend_string) -> String {
    if zs.is_null() {
        return String::new();
    }
    unsafe {
        let val_ptr = (*zs).val.as_ptr() as *const u8;
        let len = (*zs).len;
        let bytes = std::slice::from_raw_parts(val_ptr, len);
        String::from_utf8_lossy(bytes).into_owned()
    }
}
