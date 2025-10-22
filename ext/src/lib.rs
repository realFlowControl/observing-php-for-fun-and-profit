use ext_php_rs::{ffi::_zend_string, prelude::*};

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(hello_world))
}

#[allow(unused)]
unsafe fn zend_string_to_string(zs: *const _zend_string) -> String {
    if zs.is_null() {
        return String::new();
    }
    unsafe {
        let val_ptr = (*zs).val.as_ptr();
        let len = (*zs).len;
        let bytes = std::slice::from_raw_parts(val_ptr, len);
        String::from_utf8_lossy(bytes).into_owned()
    }
}
