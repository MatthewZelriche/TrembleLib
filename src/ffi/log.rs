use std::{ffi::CStr, os::raw::c_char};

#[no_mangle]
pub unsafe extern "C" fn tr_info(filename_ptr: *const c_char, line: u32, msg_ptr: *const c_char) {
    let msg = str_or_default(msg_ptr);
    let filename = str_or_default(filename_ptr);
    log::logger().log(
        &log::Record::builder()
            .args(format_args!("{}", msg))
            .level(log::Level::Info)
            .file(Some(filename))
            .line(Some(line))
            .build(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn tr_warn(filename_ptr: *const c_char, line: u32, msg_ptr: *const c_char) {
    let msg = str_or_default(msg_ptr);
    let filename = str_or_default(filename_ptr);
    log::logger().log(
        &log::Record::builder()
            .args(format_args!("{}", msg))
            .level(log::Level::Warn)
            .file(Some(filename))
            .line(Some(line))
            .build(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn tr_error(filename_ptr: *const c_char, line: u32, msg_ptr: *const c_char) {
    let msg = str_or_default(msg_ptr);
    let filename = str_or_default(filename_ptr);
    log::logger().log(
        &log::Record::builder()
            .args(format_args!("{}", msg))
            .level(log::Level::Error)
            .file(Some(filename))
            .line(Some(line))
            .build(),
    );
}

// Safety: Safe because the empty string has static lifetime (larger than a), and because the FFI caller
// has to guaruntee the pointer data lives long enough for the duration of the entire FFI call
unsafe fn str_or_default<'a>(ptr: *const c_char) -> &'a str {
    if !ptr.is_null() {
        std::str::from_utf8_unchecked(CStr::from_ptr(ptr.cast()).to_bytes())
    } else {
        ""
    }
}
