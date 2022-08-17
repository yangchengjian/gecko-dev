use ndk_sys::__android_log_print as android_log_print;

const TAG: &str = "android_ndk_in_arcore";

/// Log Debug
pub fn d(message: &str) {
    log(3, message);
}

/// Log Info
pub fn i(message: &str) {
    log(4, message);
}

/// Log Warn
pub fn w(message: &str) {
    log(5, message);
}

/// Log Error
pub fn e(message: &str) {
    log(6, message);
}


pub fn log(prio: i32, message: &str) {
    unsafe {
        android_log_print(
            prio             as std::os::raw::c_int,
            TAG.as_ptr()     as *const std::os::raw::c_char,
            message.as_ptr() as *const std::os::raw::c_char,
        );
    }
}

pub fn print_matrix(tag: &str, mat: &[f32]) {
    d(&format!("{} : {:?}", tag, mat));
}