mod ffi_arcore {
    include!(concat!(env!("OUT_DIR"), "/arcore_bindings.rs"));
}

mod augmented_face;
mod augmented_image;
mod jni_interface;
pub mod log;
mod util;

#[cfg(target_os = "android")]
extern crate glm;
extern crate jni;
extern crate jni_sys;
extern crate nalgebra_glm;
extern crate ndk;
extern crate ndk_sys;
extern crate rgb;

use std::collections::HashMap;
use jni_sys::JavaVM;
use jni_sys::JNIEnv;
use jni_sys::jobject;
use opengles::glesv2;
use crate::ffi_arcore::*;

pub const GL_TEXTURE_EXTERNAL_OES: glesv2::GLenum = 0x8D65;
pub const K_NUM_VERTICES: i32 = 4;

// UVs of the quad vertices (S, T)
const K_UVS: [f32; 8] = [
    0.0, 1.0,
    1.0, 1.0,
    0.0, 0.0,
    1.0, 0.0,
];

const K_VERS: [f32; 8] = [
    -1.0, -1.0,
    1.0, -1.0,
    -1.0, 1.0,
    1.0, 1.0,
];

const VS_SRC_CAMERA: &'static [u8] = b"
attribute vec4 a_Position;
attribute vec2 a_TexCoord;

varying vec2 v_TexCoord;

void main() {
   gl_Position = a_Position;
   v_TexCoord = a_TexCoord;
}
\0";

const FS_SRC_CAMERA: &'static [u8] = b"
#extension GL_OES_EGL_image_external : require

precision mediump float;
varying vec2 v_TexCoord;
uniform samplerExternalOES sTexture;


void main() {
    gl_FragColor = texture2D(sTexture, v_TexCoord);
}
\0";

/// initial ArCore
#[no_mangle]
pub unsafe extern "C" fn init_arcore(arcore: *mut ArCore, env: *mut JNIEnv) {
    log::i("arcore::c::init_arcore\n");

    let (env, context) = jni_interface::init_jni(env);
    (*arcore) = ArCore::new(env, context)
}

/// on surface created
#[no_mangle]
pub unsafe extern "C" fn on_surface_created(arcore: *mut ArCore) {
    log::i("arcore::c::on_surface_created\n");

    log::d(&format!("arcore::c::on_surface_created arcore before = {:?}\n", &*arcore));
    (*arcore).on_surface_created();
    log::d(&format!("arcore::c::on_surface_created arcore after = {:?}\n", &*arcore));
}

/// set display rotation, width, height
#[no_mangle]
pub unsafe extern "C" fn on_display_changed(arcore: *mut ArCore, rotation: i32, width: i32, height: i32) {
    log::i("arcore::c::on_display_changed\n");

    log::d(&format!("arcore::c::on_display_changed arcore before = {:?}\n", &*arcore));
    (*arcore).on_display_changed(rotation, width, height);
    log::d(&format!("arcore::c::on_display_changed arcore after = {:?}\n", &*arcore));
}

/// draw background and set relevant matrix
#[no_mangle]
pub unsafe extern "C" fn on_draw_frame(arcore: *mut ArCore) {
    log::i("arcore::c::on_draw_frame\n");

    log::d(&format!("arcore::c::on_draw_frame arcore before = {:?}\n", &*arcore));
    (*arcore).on_draw_frame();
    log::d(&format!("arcore::c::on_draw_frame arcore after = {:?}\n", &*arcore));
}

/// get project matrix
#[no_mangle]
pub unsafe extern "C" fn get_proj_matrix(arcore: ArCore) -> [f32; 16] {
    log::i("arcore::c::get_proj_matrix\n");
    arcore.get_proj_matrix()
}

/// get view matrix
#[no_mangle]
pub unsafe extern "C" fn get_view_matrix(arcore: ArCore) -> [f32; 16] {
    log::i("arcore::c::get_view_matrix\n");
    arcore.get_view_matrix()
}

/// get model matrix
#[no_mangle]
pub unsafe extern "C" fn get_mode_matrix(arcore: ArCore, track_type: i32, index: i32) -> [f32; 16] {
    log::i("arcore::c::get_mode_matrix\n");
    // (*arcore).get_mode_matrix(track_type, index)
    arcore.get_view_matrix()
}

/// ArAnchor Color
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ColoredAnchor {
    anchor: *mut ArAnchor,
    color: [f32; 4],
}

/// ArCore
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ArCore {
    // Surface -------------------------------------------------
    width_: i32,
    height_: i32,
    rotation_: i32,

    // ArCore -------------------------------------------------
    ar_session: *mut ArSession,
    ar_frame: *mut ArFrame,

    // Background ---------------------------------------------
    camera_program_: glesv2::GLuint,
    camera_position_attrib_: glesv2::GLuint,
    camera_tex_coord_attrib_: glesv2::GLuint,
    camera_texture_uniform_: glesv2::GLuint,
    camera_texture_id_: glesv2::GLuint,

    // depth_program_: glesv2::GLuint,
    // depth_texture_id_: glesv2::GLuint,
    // depth_texture_uniform_: glesv2::GLuint,
    // depth_position_attrib_: glesv2::GLuint,
    // depth_tex_coord_attrib_: glesv2::GLuint,

    uvs_transformed_: [f32; 8],
    uvs_initialized_: bool,

    // Object -------------------------------------------------
    show_plane: bool,
    show_point: bool,
    show_image: bool,
    show_faces: bool,

    // plane_obj_map_: HashMap<i32, ColoredAnchor>,
    // point_obj_map_: HashMap<i32, ColoredAnchor>,
    // image_obj_map_: HashMap<i32, ColoredAnchor>,
    // faces_obj_map_: HashMap<i32, ColoredAnchor>,

    // number_to_render: usize,

    // Matrix -------------------------------------------------
    view_mat4x4: [f32; 16],
    proj_mat4x4: [f32; 16],
}

impl ArCore {

    pub fn new(env: *mut JNIEnv, context: jobject) -> ArCore {
        log::i("arcore::lib::new\n");
        unsafe {
            // Create ArSession
            let mut out_session_pointer: *mut ArSession = ::std::ptr::null_mut();
            let mut ar_status_create: ArStatus = ArSession_create(env as *mut ::std::os::raw::c_void, context as *mut ::std::os::raw::c_void, &mut out_session_pointer);
            if ar_status_create != 0 {
                log::e(&format!("arcore::lib::new ArSession_create error, ar_status_create = {}\n", ar_status_create));
            }

            // Create ArConfig
            let mut out_config: *mut ArConfig = ::std::ptr::null_mut();
            ArConfig_create(out_session_pointer as *const ArSession, &mut out_config);

            // // Set Depth Mode
            // ArConfig_setDepthMode(
            //     out_session_pointer as *const ArSession,
            //     out_config,
            //     AR_DEPTH_MODE_DISABLED as i32
            // );
            //
            // // Set Instant Placement Mode
            // ArConfig_setInstantPlacementMode(
            //     out_session_pointer as *const ArSession,
            //     out_config,
            //     AR_INSTANT_PLACEMENT_MODE_DISABLED as i32
            // );

            // Create Augmented Image Database
            // let mut ar_augmented_image_database: *mut ArAugmentedImageDatabase = ::augmented_image::init_augmented_image_database(out_session_pointer as *const ArSession);
            // ArConfig_setAugmentedImageDatabase(out_session_pointer as *const ArSession, out_config, ar_augmented_image_database);
            // ArAugmentedImageDatabase_destroy(ar_augmented_image_database);

            // Check ArSession configure
            let mut ar_status_configure: ArStatus = ArSession_configure(out_session_pointer, out_config);
            if ar_status_configure != 0 {
                log::e(&format!("arcore::lib::new ArSession_configure error, ar_status_configure = {}\n", ar_status_configure));
            }
            ArConfig_destroy(out_config);

            // Create ArFrame
            let mut out_frame: *mut ArFrame = ::std::ptr::null_mut();
            ArFrame_create(out_session_pointer as *const ArSession, &mut out_frame);

            // Set Display Geometry
            ArSession_setDisplayGeometry(out_session_pointer, 0, 1, 1);

            // ArSession resume
            let mut ar_status_resume: ArStatus = ArSession_resume(out_session_pointer);
            if ar_status_resume != 0 {
                log::e(&format!("arcore::lib::new ArSession_resume error, ar_status_resume = {}\n", ar_status_resume));
            }

            ArCore {

                width_: 1,
                height_: 1,
                rotation_: 0,

                ar_session: out_session_pointer,
                ar_frame: out_frame,

                camera_program_: 0,
                camera_position_attrib_: 0,
                camera_tex_coord_attrib_: 0,
                camera_texture_uniform_: 0,
                camera_texture_id_: 0,

                // depth_program_: depth_program,
                // depth_texture_id_: depth_texture_id,
                // depth_texture_uniform_: depth_texture_uniform,
                // depth_position_attrib_: depth_position_attrib,
                // depth_tex_coord_attrib_: depth_tex_coord_attrib,

                uvs_transformed_: [0.0; 8],
                uvs_initialized_: false,

                show_plane: false,
                show_point: false,
                show_image: false,
                show_faces: false,

                // plane_obj_map_: HashMap::new(),
                // point_obj_map_: HashMap::new(),
                // image_obj_map_: HashMap::new(),
                // faces_obj_map_: HashMap::new(),
                // number_to_render: 0,

                view_mat4x4: [0.0; 16],
                proj_mat4x4: [0.0; 16],
            }
        }
    }

    pub fn get_proj_matrix(&self) -> [f32; 16] {
        // log::print_matrix("arcore::lib::::get_proj_matrix", &self.proj_mat4x4);
        self.proj_mat4x4
    }

    pub fn get_view_matrix(&self) -> [f32; 16] {
        // log::print_matrix("arcore::lib::::get_view_matrix", &self.view_mat4x4);
        self.view_mat4x4
    }

    // 1: plane, 2: point, 3: images, 4: faces
    pub fn get_mode_matrix(&self, track_type: i32, index: i32) {
        log::i(&format!("arcore::lib::get_mode_matrix track_type = {}, index = {}", track_type, index));
        // match track_type {
        //     1 => get_matrix_by_anchor_and_index(self.ar_session, &self.plane_obj_map_, index),
        //     2 => get_matrix_by_anchor_and_index(self.ar_session, &self.point_obj_map_, index),
        //     3 => get_matrix_by_anchor_and_index(self.ar_session, &self.image_obj_map_, index),
        //     4 => get_matrix_by_anchor_and_index(self.ar_session, &self.faces_obj_map_, index),
        //     _ => get_matrix_by_anchor_and_index(self.ar_session, &self.plane_obj_map_, index),
        // }
    }

    pub fn get_view_mode_matrix(&self, track_type: i32, index: i32) {
        log::i(&format!("arcore::lib::get_view_mode_matrix track_type = {}, index = {}", track_type, index));
        // let mode_mat4x4 = self.get_mode_matrix(track_type, index);
        // let vm = util::get_mat4_from_array(self.get_view_matrix()) * util::get_mat4_from_array(mode_mat4x4);
        // log::print_matrix("arcore::lib::get_view_mode_matrix", &util::get_array_from_mat4(vm));
        // util::get_array_from_mat4(vm)
    }

    pub fn get_light_estimation(&mut self) -> [f32; 4] {
        log::i("arcore::lib::light_estimation\n");
        // Get light estimation value.
        unsafe {
            let mut ar_light_estimate: *mut ArLightEstimate = ::std::ptr::null_mut();
            ArLightEstimate_create(self.ar_session as *const ArSession, &mut ar_light_estimate);
            ArFrame_getLightEstimate(self.ar_session as *const ArSession, self.ar_frame as *const ArFrame, ar_light_estimate);

            let mut ar_light_estimate_state: ArLightEstimateState = AR_LIGHT_ESTIMATE_STATE_NOT_VALID as i32;
            ArLightEstimate_getState(self.ar_session as *const ArSession,
                                     ar_light_estimate as *const ArLightEstimate,
                                     &mut ar_light_estimate_state as *mut ArLightEstimateState);

            let mut color_correction = [1.0, 1.0, 1.0, 1.0];
            if ar_light_estimate_state == AR_LIGHT_ESTIMATE_STATE_VALID as i32 {
                ArLightEstimate_getColorCorrection(self.ar_session as *const ArSession,
                                                   ar_light_estimate as *const ArLightEstimate,
                                                   color_correction.as_mut_ptr());
            }
            ArLightEstimate_destroy(ar_light_estimate);
            ar_light_estimate = ::std::ptr::null_mut();

            color_correction
        }
    }

    pub fn on_surface_created(&mut self) {
        log::i("arcore::lib::on_surface_created\n");

        self.init_renderers();
    }

    pub fn on_display_changed(&mut self, rotation: i32, width: i32, height: i32) {
        log::i(&format!("arcore::lib::on_display_changed rotation = {}, width = {}, height = {}\n", rotation, width, height));

        self.rotation_ = rotation;
        self.width_ = width;
        self.height_ = height;
        if self.ar_session != ::std::ptr::null_mut() {
            unsafe { ArSession_setDisplayGeometry(self.ar_session, rotation, width, height) };
        }
    }

    pub fn on_config_changed(&mut self, show_plane: bool, show_point: bool, show_image: bool, show_faces: bool) {
        log::i(&format!("arcore::lib::on_config_changed show_plane = {}, show_point = {}, show_image = {}, show_faces = {}", show_plane, show_point, show_image, show_faces));
        self.show_plane = show_plane;
        self.show_point = show_point;
        self.show_image = show_image;
        self.show_faces = show_faces;
    }

    pub fn on_draw_frame(&mut self) {
        log::i("arcore::lib::on_draw_frame");

        unsafe {
            // Set Camera texture
            ArSession_setCameraTextureName(self.ar_session, self.camera_texture_id_);

            // ArSession update
            let mut ar_status_update: ArStatus = ArSession_update(self.ar_session, self.ar_frame);
            if ar_status_update != 0 {
                log::e(&format!("arcore::lib::on_draw_frame ArSession_update error, ar_status_update = {}\n", ar_status_update));
                return
            }

            // Acquire Camera
            let mut out_camera: *mut ArCamera = ::std::ptr::null_mut();
            ArFrame_acquireCamera(self.ar_session as *const ArSession, self.ar_frame as *const ArFrame, &mut out_camera);
            // Camera Tracking State
            let mut camera_tracking_state: ArTrackingState = 0;
            ArCamera_getTrackingState(self.ar_session as *const ArSession, out_camera as *const ArCamera, &mut camera_tracking_state as *mut ArTrackingState);
            ArCamera_release(out_camera);
            if camera_tracking_state != AR_TRACKING_STATE_TRACKING as i32 {
                log::e(&format!("arcore::lib::on_draw_frame ArCamera_getTrackingState error, camera_tracking_state = {}\n", camera_tracking_state));
                return
            }

            // Render Background
            self.render_background();

            // Update Matrix
            self.update_proj_view_matrix();
        }
    }

    pub fn on_touched(&mut self, x: f32, y: f32) -> i32 {
        log::i("arcore::lib::on_touched\n");

        // Update loop if not hited, in onDraw
        unsafe {
            if self.ar_session != ::std::ptr::null_mut() && self.ar_frame != ::std::ptr::null_mut() {
                let mut hit_result_list: *mut ArHitResultList = ::std::ptr::null_mut();
                ArHitResultList_create(self.ar_session as *const ArSession, &mut hit_result_list);
                ArFrame_hitTest(self.ar_session as *const ArSession, self.ar_frame as *const ArFrame,
                                x, y,
                                hit_result_list);

                log::d(&format!("arcore::lib::on_touched x = {}, y = {}", x, y));

                let mut hit_result_list_size = 0;
                ArHitResultList_getSize(self.ar_session as *const ArSession,
                                        hit_result_list as *const ArHitResultList,
                                        &mut hit_result_list_size as *mut i32);

                log::d(&format!("arcore::lib::on_touched hit_result_list_size = {}", hit_result_list_size));
                let mut trackable_type: ArTrackableType = AR_TRACKABLE_NOT_VALID as i32;


                for i in 0..hit_result_list_size {
                    let mut ar_hit_result: *mut ArHitResult = ::std::ptr::null_mut();
                    ArHitResult_create(self.ar_session as *const ArSession, &mut ar_hit_result);
                    ArHitResultList_getItem(self.ar_session as *const ArSession,
                                            hit_result_list as *const ArHitResultList,
                                            i,
                                            ar_hit_result);
                    if ar_hit_result == ::std::ptr::null_mut() {
                        log::e(&format!("arcore::lib::on_touched ArHitResultList_getItem error"));
                        return -1;
                    }


                    let mut ar_trackable: *mut ArTrackable = ::std::ptr::null_mut();
                    ArHitResult_acquireTrackable(self.ar_session as *const ArSession,
                                                 ar_hit_result as *const ArHitResult,
                                                 &mut ar_trackable);

                    let mut ar_trackable_type: ArTrackableType = AR_TRACKABLE_NOT_VALID as i32;
                    ArTrackable_getType(self.ar_session as *const ArSession,
                                        ar_trackable as *const ArTrackable,
                                        &mut ar_trackable_type as *mut ArTrackableType);

                    if ar_trackable_type == AR_TRACKABLE_PLANE as i32 {
                        let mut ar_pose: *mut ArPose = ::std::ptr::null_mut();
                        ArPose_create(self.ar_session as *const ArSession, 0 as *const _, &mut ar_pose);

                        ArHitResult_getHitPose(self.ar_session as *const ArSession,
                                               ar_hit_result as *const ArHitResult,
                                               ar_pose);

                        let mut in_polygon = 0;
                        let mut ar_plane: *mut ArPlane = ::std::mem::transmute::<*mut ArTrackable, *mut ArPlane>(ar_trackable);

                        ArPlane_isPoseInPolygon(self.ar_session as *const ArSession,
                                                ar_plane as *const ArPlane,
                                                ar_pose as *const ArPose,
                                                &mut in_polygon as *mut i32);

                        ArPose_destroy(ar_pose);

                        if in_polygon != 0 {
                            log::e(&format!("arcore::lib::on_touched in_polygon = {}", in_polygon));
                            return -1;
                        }

                        let mut anchor: *mut ArAnchor = ::std::ptr::null_mut();
                        let ar_status: ArStatus = ArHitResult_acquireNewAnchor(self.ar_session,
                                                                               ar_hit_result,
                                                                               &mut anchor);
                        if ar_status != AR_SUCCESS as i32 {
                            log::e(&format!("arcore::lib::on_touched ArHitResult_acquireNewAnchor error"));
                            return -1;
                        }

                        let mut tracking_state: ArTrackingState = AR_TRACKING_STATE_STOPPED as i32;
                        ArAnchor_getTrackingState(self.ar_session as *const ArSession,
                                                  anchor as *const ArAnchor,
                                                  &mut tracking_state as *mut ArTrackingState);

                        if tracking_state != AR_TRACKING_STATE_TRACKING as i32 {
                            log::e(&format!("arcore::lib::on_touched tracking_state = {}", tracking_state));
                            ArAnchor_release(anchor);
                            return -1;
                        }


                        let mut color = [0.0, 0.0, 0.0, 0.0];
                        match trackable_type as u32 {
                            AR_TRACKABLE_POINT => {
                                color[0] = 66.0;
                                color[1] = 133.0;
                                color[2] = 244.0;
                                color[3] = 255.0;
                            }
                            AR_TRACKABLE_PLANE => {
                                color[0] = 139.0;
                                color[1] = 195.0;
                                color[2] = 74.0;
                                color[3] = 255.0;
                            }
                            _ => {
                                color[0] = 0.0;
                                color[1] = 0.0;
                                color[2] = 0.0;
                                color[3] = 0.0;
                            }
                        }

                        let colored_anchor = ColoredAnchor { anchor, color };

                        log::d(&format!("arcore::lib::on_touched i : {}, colored_anchor: {:?}", i, &colored_anchor));

                        // self.plane_obj_map_.insert(i, colored_anchor);

                        return i;
                    } else if AR_TRACKABLE_POINT as i32 == ar_trackable_type {
//                        let mut ar_point: *mut ArPoint = ::std::mem::transmute::<*mut ArTrackable, *mut ArPoint>(ar_trackable);
//                        let mut mode: ArPointOrientationMode = 0;
//
//                        ArPoint_getOrientationMode(self.ar_session as *const ArSession,
//                                                   ar_point as *const ArPoint,
//                                                   &mut mode as *mut ArPointOrientationMode);
//
//                        if mode == AR_POINT_ORIENTATION_ESTIMATED_SURFACE_NORMAL as i32 {
//                            ar_hit_result = ar_hit;
//                            trackable_type = ar_trackable_type;
//                            break;
//                        }
                        return -1;
                    }
                }

                ArHitResultList_destroy(hit_result_list);
                hit_result_list = ::std::ptr::null_mut();

                return -1;
            }
            -1
        }
    }

    pub fn on_pause(&self) {
        log::i("arcore::lib::on_pause");

        unsafe {
            if self.ar_session != ::std::ptr::null_mut() {
                ArSession_pause(self.ar_session);
            }
        }
    }

    pub fn on_finish(&self) {
        log::i("arcore::lib::on_finish");

        unsafe {
            if self.ar_session != ::std::ptr::null_mut() {
                ArSession_destroy(self.ar_session);
                ArFrame_destroy(self.ar_frame);
            }
        }
    }

    fn init_renderers(&mut self) {
        self.init_render_background();
        // self.init_render_plane();
        // self.init_render_point();
    }

    fn init_render_background(&mut self) {
        unsafe {
            // Camera
            let camera_texture_id = glesv2::gen_textures(1)[0];
            glesv2::bind_texture(GL_TEXTURE_EXTERNAL_OES, camera_texture_id);
            glesv2::tex_parameteri(GL_TEXTURE_EXTERNAL_OES, glesv2::GL_TEXTURE_MIN_FILTER, glesv2::GL_LINEAR as i32);
            glesv2::tex_parameteri(GL_TEXTURE_EXTERNAL_OES, glesv2::GL_TEXTURE_MAG_FILTER, glesv2::GL_LINEAR as i32);

            let camera_program = util::create_program(VS_SRC_CAMERA, FS_SRC_CAMERA);
            if camera_program == 0 {
                log::e("arcore::background_renderer::new Could not create camera program.\n");
            }

            let camera_position_attrib = glesv2::get_attrib_location(camera_program, "a_Position") as u32;
            let camera_tex_coord_attrib = glesv2::get_attrib_location(camera_program, "a_TexCoord") as u32;
            let camera_texture_uniform = glesv2::get_uniform_location(camera_program, "sTexture") as u32;

            self.camera_program_ = camera_program;
            self.camera_position_attrib_ = camera_position_attrib;
            self.camera_tex_coord_attrib_ = camera_tex_coord_attrib;
            self.camera_texture_uniform_ = camera_texture_uniform;
            self.camera_texture_id_ = camera_texture_id;
        }
    }

    fn init_render_plane(&mut self) {

    }

    fn init_render_point(&mut self) {

    }

    // private functions
    fn render_background(&mut self) {
        log::i("arcore::lib::render_background\n");
        unsafe {
            let mut x = 0;
            let geometry_changed: *mut i32 = &mut x;
            ArFrame_getDisplayGeometryChanged(self.ar_session, self.ar_frame, geometry_changed);

            if *geometry_changed != 0 || !self.uvs_initialized_ {
                ArFrame_transformCoordinates2d(
                    self.ar_session,
                    self.ar_frame,
                    AR_COORDINATES_2D_OPENGL_NORMALIZED_DEVICE_COORDINATES as i32,
                    K_NUM_VERTICES * 2,
                    &K_VERS as *const f32,
                    AR_COORDINATES_2D_TEXTURE_NORMALIZED as i32,
                    self.uvs_transformed_.as_mut_ptr(),
                );
                self.uvs_initialized_ = true;
            }

            // let mut frame_timestamp: i64 = 0;
            // ArFrame_getTimestamp(session, frame, &mut frame_timestamp as *mut i64);
            // if frame_timestamp == 0 {
            //     return
            // }

            // if depth_texture_id_ == -1 || camera_texture_id_ == -1 {
            //     return
            // }

            // log::d(&format!("arcore::lib::render_background camera_program : {}\n", &self.camera_program_));
            // log::d(&format!("arcore::lib::render_background camera_texture_uniform : {}\n", &self.camera_texture_uniform_));
            // log::d(&format!("arcore::lib::render_background camera_position_attrib : {}\n", &self.camera_position_attrib_));
            // log::d(&format!("arcore::lib::render_background camera_tex_coord_attrib : {}\n", &self.camera_tex_coord_attrib_));

            glesv2::use_program(self.camera_program_);
            glesv2::depth_mask(false);

            glesv2::uniform1i(self.camera_texture_uniform_ as i32, 1);
            glesv2::active_texture(glesv2::GL_TEXTURE1);
            glesv2::bind_texture(GL_TEXTURE_EXTERNAL_OES, self.camera_texture_id_);

            // let vbo = glesv2::gen_buffers(2);
            //
            // glesv2::bind_buffer(glesv2::GL_ARRAY_BUFFER, vbo[0]);
            // glesv2::buffer_data(glesv2::GL_ARRAY_BUFFER, &K_VERS, glesv2::GL_STATIC_DRAW);
            // glesv2::enable_vertex_attrib_array(self.camera_position_attrib_);
            // glesv2::vertex_attrib_pointer(self.camera_position_attrib_, 2, glesv2::GL_FLOAT, false, 0, &[0]);
            //
            // glesv2::bind_buffer(glesv2::GL_ARRAY_BUFFER, vbo[1]);
            // glesv2::buffer_data(glesv2::GL_ARRAY_BUFFER, &self.uvs_transformed_, glesv2::GL_STATIC_DRAW);
            // glesv2::enable_vertex_attrib_array(self.camera_tex_coord_attrib_);
            // glesv2::vertex_attrib_pointer(self.camera_tex_coord_attrib_, 2, glesv2::GL_FLOAT, false, 0, &[0]);
            //
            // glesv2::draw_arrays(glesv2::GL_TRIANGLE_STRIP, 0, 4);

            glesv2::enable_vertex_attrib_array(self.camera_position_attrib_);
            glesv2::vertex_attrib_pointer(self.camera_position_attrib_, 2, glesv2::GL_FLOAT, false, 0, &K_VERS);

            glesv2::enable_vertex_attrib_array(self.camera_tex_coord_attrib_);
            glesv2::vertex_attrib_pointer(self.camera_tex_coord_attrib_, 2, glesv2::GL_FLOAT, false, 0, &self.uvs_transformed_);

            glesv2::draw_arrays(glesv2::GL_TRIANGLE_STRIP, 0, 4);

            glesv2::use_program(0);
            glesv2::depth_mask(true);
        }
    }

    fn render_point_cloud(&mut self, mvp_matrix: ::glm::Mat4) {
        log::i("arcore::lib::render_point_cloud\n");
        // Update and render point cloud.
        unsafe {
            let mut ar_point_cloud: *mut ArPointCloud = ::std::ptr::null_mut();
            let point_cloud_status =
                ArFrame_acquirePointCloud(self.ar_session as *const ArSession,
                                          self.ar_frame as *const ArFrame,
                                          &mut ar_point_cloud);

            if point_cloud_status == AR_SUCCESS as i32 {
                // self.clone().renderer_point_cloud_.unwrap().draw(mvp_matrix, self.ar_session, ar_point_cloud);
                ArPointCloud_release(ar_point_cloud);
            }
        }
    }

    fn render_planes(&mut self) {
        log::i("arcore::lib::render_planes\n");
        // Update loop, in onDraw
        unsafe {

            // Update and render planes.
            let mut plane_list: *mut ArTrackableList = ::std::ptr::null_mut();
            ArTrackableList_create(self.ar_session as *const ArSession, &mut plane_list);
            if plane_list == ::std::ptr::null_mut() {
                log::e("arcore::lib::render_planes plane_list is null");
            }

            let plane_tracked_type: ArTrackableType = AR_TRACKABLE_PLANE as i32;
            ArSession_getAllTrackables(self.ar_session as *const ArSession, plane_tracked_type, plane_list);

            let mut plane_list_size = 0;
            ArTrackableList_getSize(self.ar_session as *const ArSession,
                                    plane_list as *const ArTrackableList,
                                    &mut plane_list_size as *mut i32);

            log::i(&format!("arcore::lib::render_planes plane_list_size : {:?}", plane_list_size));

            for i in 0..plane_list_size {
                let mut ar_trackable: *mut ArTrackable = ::std::ptr::null_mut();
                ArTrackableList_acquireItem(self.ar_session as *const ArSession,
                                            plane_list as *const ArTrackableList,
                                            i,
                                            &mut ar_trackable);
                let ar_plane: *mut ArPlane = ::std::mem::transmute::<*mut ArTrackable, *mut ArPlane>(ar_trackable);
                let mut out_tracking_state: ArTrackingState = 0;
                ArTrackable_getTrackingState(self.ar_session as *const ArSession,
                                             ar_trackable as *const ArTrackable,
                                             &mut out_tracking_state as *mut ArTrackingState);
                let mut subsume_plane: *mut ArPlane = ::std::ptr::null_mut();
                ArPlane_acquireSubsumedBy(self.ar_session as *const ArSession,
                                          ar_plane as *const ArPlane,
                                          &mut subsume_plane);
                if subsume_plane != ::std::ptr::null_mut() {
                    ArTrackable_release(::std::mem::transmute::<*mut ArPlane, *mut ArTrackable>(subsume_plane));
                    continue;
                }

                if out_tracking_state != AR_TRACKING_STATE_TRACKING as i32 {
                    continue;
                }

                let p = util::get_mat4_from_array(self.proj_mat4x4);
                let v = util::get_mat4_from_array(self.view_mat4x4);

                // self.clone().renderer_plane_.unwrap().draw(gl, p, v, self.ar_session, ar_plane, ::glm::vec3(255.0, 255.0, 255.0));
            }

            ArTrackableList_destroy(plane_list);
            plane_list = ::std::ptr::null_mut();
        }
    }

    fn update_proj_view_matrix(&mut self) {
        log::i("arcore::lib::update_proj_view_matrix\n");
        unsafe {
            let mut out_camera: *mut ArCamera = ::std::ptr::null_mut();
            ArFrame_acquireCamera(self.ar_session as *const ArSession,
                                  self.ar_frame as *const ArFrame,
                                  &mut out_camera);

            ArCamera_getProjectionMatrix(self.ar_session as *const ArSession,
                                         out_camera as *const ArCamera,
                                         0.1,
                                         100.0,
                                         self.proj_mat4x4.as_mut_ptr());
            ArCamera_getViewMatrix(self.ar_session as *const ArSession,
                                   out_camera as *const ArCamera,
                                   self.view_mat4x4.as_mut_ptr());

            ArCamera_release(out_camera);

            // let p = util::get_mat4_from_array(self.proj_mat4x4);
            // let v = util::get_mat4_from_array(self.view_mat4x4);
        }
    }
}


fn get_matrix_by_anchor_and_index(session: *mut ArSession, obj_map: &HashMap<i32, ColoredAnchor>, index: i32) -> [f32; 16] {
    let mut mode_mat4x4 = [0.0; 16];
    match obj_map.get(&index) {
        Some(colored_anchor) => {
            let mut tracking_state: ArTrackingState = AR_TRACKING_STATE_STOPPED as i32;
            unsafe { ArAnchor_getTrackingState(session as *const ArSession, colored_anchor.anchor, &mut tracking_state as *mut ArTrackingState) };
            if tracking_state == AR_TRACKING_STATE_TRACKING as i32 {
                get_transform_matrix_from_anchor(session, colored_anchor.anchor, mode_mat4x4.as_mut_ptr())
            }
        }
        None => {}
    }
    log::print_matrix("arcore::lib::::get_mode_matrix", &mode_mat4x4);
    mode_mat4x4
}

fn get_transform_matrix_from_anchor(session: *mut ArSession, anchor: *mut ArAnchor, out_model_mat: *mut f32) {
    unsafe {
        if out_model_mat == ::std::ptr::null_mut() {
            return;
        }
        let mut out_pose: *mut ArPose = 0 as *mut _;
        ArPose_create(session as *const ArSession, 0 as *const _, &mut out_pose);
        ArAnchor_getPose(session as *const ArSession, anchor as *const ArAnchor, out_pose as *mut ArPose);
        ArPose_getMatrix(session as *const ArSession, out_pose as *const ArPose, out_model_mat);
    }
}

fn get_ar_image_from_camera(session: *const ArSession, frame: *const ArFrame) -> *mut ArImage {
    unsafe {
        let mut ar_image: *mut ArImage = ::std::ptr::null_mut();
        let ar_status: ArStatus = ArFrame_acquireCameraImage(session as *mut ArSession, frame as *mut ArFrame, &mut ar_image);
        if ar_status != AR_SUCCESS as i32 {
            log::d(&format!("arcore::lib::get_ar_image_from_camera ar_status : {:?}", &ar_status));
        }
        ar_image
    }
}

// java.lang.UnsatisfiedLinkError: dlopen failed: cannot locate symbol "ArImage_getFormat"
fn get_ar_image_format(session: *const ArSession, ar_image: *const ArImage) -> i32 {
    unsafe {
        let mut height = 0;
        ArImage_getFormat(session, ar_image, &mut height as *mut i32);
        height
    }
}

fn get_ar_image_height(session: *const ArSession, ar_image: *const ArImage) -> i32 {
    unsafe {
        let mut height = 0;
        ArImage_getHeight(session, ar_image, &mut height as *mut i32);
        height
    }
}

fn get_ar_image_width(session: *const ArSession, ar_image: *const ArImage) -> i32 {
    unsafe {
        let mut width = 0;
        ArImage_getWidth(session, ar_image, &mut width as *mut i32);
        width
    }
}

fn get_ar_image_timestamp(session: *const ArSession, ar_image: *const ArImage) -> i64 {
    unsafe {
        let mut timestamp = 0;
        ArImage_getTimestamp(session, ar_image, &mut timestamp as *mut i64);
        timestamp
    }
}