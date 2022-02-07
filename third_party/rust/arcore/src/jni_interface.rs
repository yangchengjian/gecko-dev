use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;

use jni_sys::JavaVM;
use jni_sys::JNIEnv;
use jni_sys::jclass;
use jni_sys::jint;
use jni_sys::jobject;
// use ndk_sys::AASSET_MODE_BUFFER;
// use ndk_sys::AAsset;
// use ndk_sys::AAsset_read;
// use ndk_sys::AAssetManager;
// use ndk_sys::AAssetManager_open;
// use ndk_sys::ANativeActivity;

use crate::log;

// static mut gJavaVm: *mut JavaVM = std::ptr::null_mut();
// static mut STATIC_JNI_ENV: *mut JNIEnv = std::ptr::null_mut();

// #[no_mangle]
// #[allow(non_snake_case)]
// unsafe fn JNI_OnLoad(jvm: *mut JavaVM, _reserved: *mut c_void) -> jint {
//     log::i("arcore::jni_interface::JNI_OnLoad");
//     STATIC_JVM = jvm;
//     0
// }
//
// #[no_mangle]
// #[allow(non_snake_case)]
// unsafe fn JNI_OnUnload(jvm: *mut JavaVM, _reserved: *mut c_void) {
//     log::i("arcore::jni_interface::JNI_OnUnload");
// }

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

// /// Load image from assets
// pub fn load_image_from_assets(path: &str,
//                               out_width: &mut u32,
//                               out_height: &mut u32,
//                               out_stride: &mut u32,
//                               out_pixel_buffer: *mut *mut u8) -> bool {
//     log::d("arcore::jni_interface::load_image_from_assets");
//
//     let image_obj = call_java_load_image(path);
//     log::d(&format!("arcore::jni_interface::load_image_from_assets image_obj =  {:?}", &image_obj));
//
//     let android_bitmap_getinfo = AndroidBitmap_getInfo;
//     let android_bitmap_lockpixels = AndroidBitmap_lockPixels;
//     let android_bitmap_unlockpixels = AndroidBitmap_unlockPixels;
//
//     let env = get_jni_env();
//
//     // image_obj contains a Bitmap Java object.
//     let mut bitmap_info = AndroidBitmapInfo { width: 0, height: 0, stride: 0, format: 0, flags: 0 };
//
//     unsafe { android_bitmap_getinfo(env, image_obj, &bitmap_info as *const AndroidBitmapInfo as *mut AndroidBitmapInfo); }
//
//     log::e(&format!("arcore::jni_interface::load_image_from_assets bitmap_info =  {:?}", &bitmap_info as *const AndroidBitmapInfo));
//
//     // Attention: We are only going to support RGBA_8888 format in this sample.
//
//     if bitmap_info.format == ANDROID_BITMAP_FORMAT_RGBA_8888 {
//         *out_width = bitmap_info.width;
//         *out_height = bitmap_info.height;
//         *out_stride = bitmap_info.stride;
//
//         let mut jvm_buffer: *mut c_void = std::ptr::null_mut();
//
//         unsafe {
//             if android_bitmap_lockpixels(env, image_obj, &mut jvm_buffer as *mut *mut c_void) != 0 {
//                 log::e("arcore::jni_interface::load_image_from_assets android_bitmap_lockpixels failed");
//                 return false;
//             }
//         }
//
//         log::d(&format!("arcore::jni_interface::load_image_from_assets jvm_buffer =  {:?}", &jvm_buffer));
//
//         let buf_size = bitmap_info.width * bitmap_info.stride;
//
//         unsafe { *out_pixel_buffer = jvm_buffer as *mut u8 };
//
//         log::d(&format!("arcore::jni_interface::load_image_from_assets pixel_buffer =  {:?}", &out_pixel_buffer));
//
//         // release jvm_buffer back to JVM
//         unsafe {
//             if android_bitmap_unlockpixels(env, image_obj) != 0 {
//                 log::e("arcore::jni_interface::load_image_from_assets android_bitmap_unlockpixels failed");
//                 return false;
//             }
//         }
//     } else {
//         return false;
//     }
//     true
// }
//
//
// pub fn load_png_from_assets(target: i32, path: &str) -> bool {
//     let env = get_jni_env();
//
//     let image_obj = call_java_load_image(path);
//     log::d(&format!("arcore::jni_interface::load_png_from_assets image_obj =  {:?}", &image_obj));
//     unsafe {
//         call_static_void_method(JNI_CLASS_JNI_INTERFACE_ID, JNI_METHOD_LOAD_TEXTURE_ID, target, image_obj);
//     }
//     true
// }

//
// fn call_java_load_image(path: &str) -> jobject {
//     log::d(&format!("arcore::jni_interface::call_java_load_image path = {}", path));
//
//
//     let jni_path = new_string_utf(path);
//     log::e(&format!("arcore::jni_interface::call_java_load_image jni_path =  {:?}", &jni_path));
//
//     unsafe { call_static_object_method(JNI_CLASS_JNI_INTERFACE_ID, JNI_METHOD_LOAD_IMAGE_ID, jni_path) }
// }
//
// fn get_jni_env() -> *mut JNIEnv {
//     log::d("arcore::jni_interface::get_jni_env");
//
//     let mut env: *mut JNIEnv = std::ptr::null_mut();
//     let jni_invoke_interface: *const JNIInvokeInterface = unsafe { (*STATIC_JVM).functions };
//     let attach = unsafe { (*jni_invoke_interface).AttachCurrentThread };
//     unsafe { attach(STATIC_JVM, &mut env, std::ptr::null_mut()) };
//
//     log::d(&format!("arcore::jni_interface::get_jni_env env =  {:?}", &env));
//
//     env
// }
//
// fn get_version() -> jint {
//     let env = get_jni_env();
//     let jni_native_interface: *const JNINativeInterface = unsafe { (*env).functions };
//
//     let get_version_0 = unsafe { (*jni_native_interface).GetVersion };
//     get_version_0(env)
// }
//
// fn find_class(class_name: &str) -> jclass {
//     let env = get_jni_env();
//     let jni_native_interface: *const JNINativeInterface = unsafe { (*env).functions };
//
//     let c_str_class_name = CString::new(class_name).unwrap();
//
//     let find_class_0 = unsafe { (*jni_native_interface).FindClass };
//     let class_id = find_class_0(env, c_str_class_name.as_ptr() as *const c_char);
//
//     let new_global_ref_0 = unsafe { (*jni_native_interface).NewGlobalRef };
//     new_global_ref_0(env, class_id)
// }
//
// fn get_static_method_id(class_id: jclass, method_name: &str, method_args: &str) -> jmethodID {
//     let env = get_jni_env();
//     let jni_native_interface: *const JNINativeInterface = unsafe { (*env).functions };
//
//     let c_str_method_name = CString::new(method_name).unwrap();
//     let c_str_method_args = CString::new(method_args).unwrap();
//
//     let get_static_method_id_0 = unsafe { (*jni_native_interface).GetStaticMethodID };
//     get_static_method_id_0(env, class_id, c_str_method_name.as_ptr() as *const c_char, c_str_method_args.as_ptr() as *const c_char)
// }
//
// fn new_string_utf(raw: &str) -> jstring {
//     let env = get_jni_env();
//     let jni_native_interface: *const JNINativeInterface = unsafe { (*env).functions };
//
//     let c_str_raw = CString::new(raw).unwrap();
//
//     let new_string_utf_0 = unsafe { (*jni_native_interface).NewStringUTF };
//     new_string_utf_0(env, c_str_raw.as_ptr() as *const c_char)
// }
//
// fn call_static_object_method(class_id: jclass, method_id: jmethodID, args: jstring) -> jobject {
//     let env = get_jni_env();
//     let jni_native_interface: *const JNINativeInterface = unsafe { (*env).functions };
//
//     let call_static_object_method_0 = unsafe { (*jni_native_interface).CallStaticObjectMethod };
//     call_static_object_method_0(env, class_id, method_id, args)
// }
//
// fn call_static_void_method(class_id: jclass, method_id: jmethodID, arg1: i32, arg2: jobject) {
//     let env = get_jni_env();
//     let jni_native_interface: *const JNINativeInterface = unsafe { (*env).functions };
//
//     let call_static_void_method_0 = unsafe { (*jni_native_interface).CallStaticVoidMethod };
//     call_static_void_method_0(env, class_id, method_id, arg1, arg2)
// }
//
// fn get_asset_manager() -> *mut AAssetManager {
//     log::d("arcore::jinterface::get_asset_manager");
//
//     unsafe {
//         let app: &mut android_app = get_app();
//         let activity: *const ANativeActivity = (*app).activity;
//
// //        let vm: *mut JavaVM = unsafe { (*activity).vm };
// //        let mut env: *mut JNIEnv = unsafe { (*activity).env };
// //
// //        let jni_invoke_interface: *const JNIInvokeInterface = unsafe { (*vm).functions };
// //        let attach = unsafe { (*jni_invoke_interface).AttachCurrentThread };
// //        attach(vm, &mut env, std::ptr::null_mut());
//
//         (*activity).assetManager
//     }
// }
//
// fn get_asset(file: &str) -> *mut AAsset {
//     log::d("arcore::jinterface::get_asset");
//     unsafe {
//         let c_str_file = CString::new(file).unwrap();
//
//         let aasset_manager_open = AAssetManager_open;
//         aasset_manager_open(get_asset_manager(), c_str_file.as_ptr() as *const c_char, AASSET_MODE_BUFFER)
//     }
// }
//
// fn asset_read(file: &str) -> Vec<u8> {
//     log::d("arcore::jinterface::asset_read");
//     unsafe {
//         let mut result = Vec::new();
//
//         let aas = get_asset(file);
//         let size = 1024;
//         let mut buf: Vec<u8> = Vec::with_capacity(size);
//         let mut need_loop = true;
//         let aasset_read = AAsset_read;
//         while need_loop {
//             let nb_read = aasset_read(aas, buf.as_mut_ptr() as *mut c_void, size);
//             if nb_read > 0 {
//                 buf.set_len(nb_read as usize);
//                 result.append(&mut buf.to_vec());
//             } else {
//                 need_loop = false
//             }
//         }
//         result
//     }
// }