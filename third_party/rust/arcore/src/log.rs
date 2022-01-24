use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

// use log::Level;
use ndk_glue::android_log;

// const TAG: &CStr = "AndroidNkdGlue" as raw::c_char;

fn get_c_char(s: &str) -> *const c_char {
    let c_str = CString::new(s).unwrap();
    c_str.as_ptr()
}

/// Log Debug
pub fn d(message: &str) {
    log(4, message);
}

/// Log Info
pub fn i(message: &str) {
    log(3, message);
}

/// Log Warn
pub fn w(message: &str) {
    log(2, message);
}

/// Log Error
pub fn e(message: &str) {
    log(1, message);
}


pub fn log(level: i32, message: &str) {
    // android_log(level, TAG, CStr::from_ptr(message));
}

pub fn print_matrix(tag: &str, mat: &[f32]) {
    d(&format!("{} : {:?}", tag, mat));
}