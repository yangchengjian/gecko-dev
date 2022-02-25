use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;

use jni_sys::JavaVM;
use jni_sys::JNIEnv;
use jni_sys::jclass;
use jni_sys::jint;
use jni_sys::jobject;

use crate::log;

/// Init Jni to get Env and jobject
pub fn init_jni(env: *mut JNIEnv) -> (*mut JNIEnv, jobject) {
    log::d("arcore::jni_interface::init_jni\n");

    // Get JavaVM
    // let get_java_vm = unsafe { (*(*env)).GetJavaVM.unwrap() };
    // unsafe { get_java_vm(env, gJavaVm as *mut *mut JavaVM) };

    // Get JNIEnv
    // let mut env: *mut JNIEnv = std::ptr::null_mut();
    // let get_env = unsafe { (*(*gJavaVm)).GetEnv.unwrap() };
    // unsafe { get_env(STATIC_JVM.clone(), env as *mut *mut c_void, jni_sys::JNI_VERSION_1_8) };
    // let attach = unsafe { (*(*gJavaVm)).AttachCurrentThread.unwrap() };
    // unsafe { attach(gJavaVm, env as *mut *mut c_void, std::ptr::null_mut()) };
    log::d(&format!("arcore::jni_interface::init_jni env =  {:?}\n", &env));

    // Get jobject
    // jclass activity_thread_class = (*env)->FindClass(env,"android/app/ActivityThread");
    // jmethodID activity_thread_method_id = (*env)->GetStaticMethodID(env,activity_thread_class, "currentActivityThread", "()Landroid/app/ActivityThread;");
    // jobject activity_thread_object = (*env)->CallStaticObjectMethod(env,activity_thread_class, activity_thread_method_id);
    //
    // jmethodID getApplication = (*env)->GetMethodID(env,activity_thread_class, "getApplication", "()Landroid/app/Application;");
    // jobject context = (*env)->CallObjectMethod(env,activity_thread_object, getApplication);
    // return context;

    // Get functions
    let find_class = unsafe { (*(*env)).FindClass.unwrap() };
    let get_method_id = unsafe { (*(*env)).GetMethodID.unwrap() };
    let get_static_method_id = unsafe { (*(*env)).GetStaticMethodID.unwrap() };
    let call_object_method = unsafe { (*(*env)).CallObjectMethod.unwrap() };
    let call_static_object_method = unsafe { (*(*env)).CallStaticObjectMethod.unwrap() };

    // Exec fuctions
    let mut context: jobject = std::ptr::null_mut();
    let mut activity_thread_class = unsafe { find_class(env, CString::new("android/app/ActivityThread").unwrap().as_ptr() as *const c_char) };
    let mut activity_thread_method_id = unsafe { get_static_method_id(env, activity_thread_class, CString::new("currentActivityThread").unwrap().as_ptr() as *const c_char, CString::new("()Landroid/app/ActivityThread;").unwrap().as_ptr() as *const c_char) };
    let mut activity_thread_object = unsafe { call_static_object_method(env, activity_thread_class, activity_thread_method_id) };
    let mut get_application_method_id = unsafe { get_method_id(env, activity_thread_class, CString::new("getApplication").unwrap().as_ptr() as *const c_char, CString::new("()Landroid/app/Application;").unwrap().as_ptr() as *const c_char) };
    context = unsafe { call_object_method(env, activity_thread_object, get_application_method_id) };
    log::d(&format!("arcore::jni_interface::init_jni obj =  {:?}\n", &context));

    (env, context)
}