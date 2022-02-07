use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::rc::Rc;
use libc::c_char;

use opengles::glesv2;

use crate::ffi_arcore::*;
use crate::log;
use crate::util;

pub const GL_TEXTURE_EXTERNAL_OES: glesv2::GLenum = 0x8D65;
pub const K_NUM_VERTICES: i32 = 4;


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

const VS_SRC_DEPTH: &'static [u8] = b"
attribute vec4 a_Position;
attribute vec2 a_TexCoord;

varying vec2 v_TexCoord;

void main() {
   v_TexCoord = a_TexCoord;
   gl_Position = a_Position;
}
\0";

const FS_SRC_DEPTH: &'static [u8] = b"
precision mediump float;

uniform sampler2D u_DepthTexture;

varying vec2 v_TexCoord;

const highp float kMaxDepth = 8000.0; // In millimeters.

float DepthGetMillimeters(in sampler2D depth_texture, in vec2 depth_uv) {
  // Depth is packed into the red and green components of its texture.
  // The texture is a normalized format, storing millimeters.
  vec3 packedDepthAndVisibility = texture2D(depth_texture, depth_uv).xyz;
  return dot(packedDepthAndVisibility.xy, vec2(255.0, 256.0 * 255.0));
}

// Returns a color corresponding to the depth passed in. Colors range from red
// to green to blue, where red is closest and blue is farthest.
//
// Uses Turbo color mapping:
// https://ai.googleblog.com/2019/08/turbo-improved-rainbow-colormap-for.html
vec3 DepthGetColorVisualization(in float x) {
  const vec4 kRedVec4 = vec4(0.55305649, 3.00913185, -5.46192616, -11.11819092);
  const vec4 kGreenVec4 = vec4(0.16207513, 0.17712472, 15.24091500, -36.50657960);
  const vec4 kBlueVec4 = vec4(-0.05195877, 5.18000081, -30.94853351, 81.96403246);
  const vec2 kRedVec2 = vec2(27.81927491, -14.87899417);
  const vec2 kGreenVec2 = vec2(25.95549545, -5.02738237);
  const vec2 kBlueVec2 = vec2(-86.53476570, 30.23299484);
  const float kInvalidDepthThreshold = 0.01;

  // Adjusts color space via 6 degree poly interpolation to avoid pure red.
  x = clamp(x * 0.9 + 0.03, 0.0, 1.0);
  vec4 v4 = vec4(1.0, x, x * x, x * x * x);
  vec2 v2 = v4.zw * v4.z;
  vec3 polynomial_color = vec3(
    dot(v4, kRedVec4) + dot(v2, kRedVec2),
    dot(v4, kGreenVec4) + dot(v2, kGreenVec2),
    dot(v4, kBlueVec4) + dot(v2, kBlueVec2)
  );

  return step(kInvalidDepthThreshold, x) * polynomial_color;
}

void main() {
  highp float normalized_depth =
      clamp(DepthGetMillimeters(u_DepthTexture, v_TexCoord.xy) / kMaxDepth,
            0.0, 1.0);
  vec4 depth_color = vec4(DepthGetColorVisualization(normalized_depth), 1.0);
  gl_FragColor = depth_color;
}
\0";

const K_VERTICES: [f32; 12] = [
    -1.0, -1.0, 0.0,
    1.0, -1.0, 0.0,
    -1.0, 1.0, 0.0,
    1.0, 1.0, 0.0,
];

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
    1.0, 0.0,
];

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BackgroundRenderer {
    camera_program_: glesv2::GLuint,
    camera_texture_id_: glesv2::GLuint,
    camera_position_attrib_: glesv2::GLuint,
    camera_tex_coord_attrib_: glesv2::GLuint,
    camera_texture_uniform_: glesv2::GLuint,

    // depth_program_: glesv2::GLuint,
    // depth_texture_id_: glesv2::GLuint,
    // depth_texture_uniform_: glesv2::GLuint,
    // depth_position_attrib_: glesv2::GLuint,
    // depth_tex_coord_attrib_: glesv2::GLuint,

    transformed_uvs_: [f32; 8],
    uvs_initialized_: bool,
}

impl BackgroundRenderer {
    pub fn new() -> BackgroundRenderer {
        log::i("arcore::background_renderer::new\n");
        unsafe {

            let camera_texture_id = glesv2::gen_textures(1)[0];
            glesv2::bind_texture(GL_TEXTURE_EXTERNAL_OES, camera_texture_id);
            glesv2::tex_parameteri(GL_TEXTURE_EXTERNAL_OES, glesv2::GL_TEXTURE_MIN_FILTER, glesv2::GL_LINEAR as i32);
            glesv2::tex_parameteri(GL_TEXTURE_EXTERNAL_OES, glesv2::GL_TEXTURE_MAG_FILTER, glesv2::GL_LINEAR as i32);

            let camera_program = util::create_program(VS_SRC_CAMERA, FS_SRC_CAMERA);
            if camera_program == 0 {
                log::e("arcore::background_renderer::new Could not create camera program.\n");
            }
            let camera_texture_uniform  = glesv2::get_uniform_location(camera_program, "sTexture") as u32;
            let camera_position_attrib  = glesv2::get_attrib_location(camera_program, "a_Position") as u32;
            let camera_tex_coord_attrib = glesv2::get_attrib_location(camera_program, "a_TexCoord") as u32;

            // let depth_program = util::create_program(VS_SRC_DEPTH, FS_SRC_DEPTH);
            // if depth_program == 0 {
            //     log::e("arcore::background_renderer::new Could not create depth program.\n");
            // }
            // let depth_texture_uniform  = glesv2::get_uniform_location(depth_program, "u_DepthTexture") as u32;
            // let depth_position_attrib  = glesv2::get_attrib_location(depth_program, "a_Position") as u32;
            // let depth_tex_coord_attrib = glesv2::get_attrib_location(depth_program, "a_TexCoord") as u32;

            // depth_texture_id_ = depth_texture_id;

            let transformed_uvs: [f32; 8] = [0.0; 8];

            log::d(&format!("arcore::background_renderer::new camera_program : {}\n", &camera_program));
            log::d(&format!("arcore::background_renderer::new camera_texture_uniform : {}\n", &camera_texture_uniform));
            log::d(&format!("arcore::background_renderer::new camera_position_attrib : {}\n", &camera_position_attrib));
            log::d(&format!("arcore::background_renderer::new camera_tex_coord_attrib : {}\n", &camera_tex_coord_attrib));

            BackgroundRenderer {
                camera_program_: camera_program,
                camera_texture_id_: camera_texture_id,

                camera_position_attrib_: camera_position_attrib,
                camera_tex_coord_attrib_: camera_tex_coord_attrib,
                camera_texture_uniform_: camera_texture_uniform,

                // depth_program_: depth_program,
                // depth_texture_id_: depth_texture_id,
                // depth_texture_uniform_: depth_texture_uniform,
                // depth_position_attrib_: depth_position_attrib,
                // depth_tex_coord_attrib_: depth_tex_coord_attrib,

                transformed_uvs_: transformed_uvs,
                uvs_initialized_: false,
            }
        }
    }

    pub fn draw(&mut self, session: *const ArSession, frame: *const ArFrame) {
        log::d("arcore::background_renderer::draw");
        unsafe {
            let mut x = 0;
            let geometry_changed: *mut i32 = &mut x;
            ArFrame_getDisplayGeometryChanged(session, frame, geometry_changed);

            log::d(&format!("arcore::background_renderer::draw geometry_changed : {}\n", *geometry_changed));

            if *geometry_changed != 0 || !self.uvs_initialized_ {
                ArFrame_transformCoordinates2d(
                    session,
                    frame,
                    AR_COORDINATES_2D_OPENGL_NORMALIZED_DEVICE_COORDINATES as i32,
                    K_NUM_VERTICES * 2,
                    &K_VERS as *const f32,
                    AR_COORDINATES_2D_TEXTURE_NORMALIZED as i32,
                    self.transformed_uvs_.as_mut_ptr()
                );
                self.uvs_initialized_ = true;
                log::d(&format!("arcore::background_renderer::draw self.uvs_initialized_ : {}\n", self.uvs_initialized_));
            }

            // let mut frame_timestamp: i64 = 0;
            // ArFrame_getTimestamp(session, frame, &mut frame_timestamp as *mut i64);
            // if frame_timestamp == 0 {
            //     return
            // }

            // if depth_texture_id_ == -1 || camera_texture_id_ == -1 {
            //     return
            // }

            log::d(&format!("arcore::background_renderer::draw camera_program : {}\n", &self.camera_program_));
            log::d(&format!("arcore::background_renderer::draw camera_texture_uniform : {}\n", &self.camera_texture_uniform_));
            log::d(&format!("arcore::background_renderer::draw camera_position_attrib : {}\n", &self.camera_position_attrib_));
            log::d(&format!("arcore::background_renderer::draw camera_tex_coord_attrib : {}\n", &self.camera_tex_coord_attrib_));

            glesv2::use_program(self.camera_program_);
            glesv2::depth_mask(false);

            glesv2::uniform1i(self.camera_texture_uniform_ as i32, 1);
            glesv2::active_texture(glesv2::GL_TEXTURE1);
            glesv2::bind_texture(GL_TEXTURE_EXTERNAL_OES, self.camera_texture_id_);

            let vbo = glesv2::gen_buffers(2);

            glesv2::bind_buffer(glesv2::GL_ARRAY_BUFFER, vbo[0]);
            glesv2::buffer_data(glesv2::GL_ARRAY_BUFFER, &K_VERTICES, glesv2::GL_STATIC_DRAW);

            glesv2::enable_vertex_attrib_array(self.camera_position_attrib_);
            glesv2::vertex_attrib_pointer(self.camera_position_attrib_, 3, glesv2::GL_FLOAT, false, 0, &[0]);


            glesv2::bind_buffer(glesv2::GL_ARRAY_BUFFER, vbo[1]);
            glesv2::buffer_data(glesv2::GL_ARRAY_BUFFER, &self.transformed_uvs_, glesv2::GL_STATIC_DRAW);

            glesv2::enable_vertex_attrib_array(self.camera_tex_coord_attrib_);
            glesv2::vertex_attrib_pointer(self.camera_tex_coord_attrib_, 2, glesv2::GL_FLOAT, false, 0, &[0]);

            glesv2::draw_arrays(glesv2::GL_TRIANGLE_STRIP, 0, 4);

            // glesv2::vertex_attrib_pointer(self.camera_position_attrib_, 2, glesv2::GL_FLOAT, false, 0, &K_VERS);
            // glesv2::vertex_attrib_pointer(self.camera_tex_coord_attrib_, 2, glesv2::GL_FLOAT, false, 0, &self.transformed_uvs_);
            //
            // glesv2::enable_vertex_attrib_array(self.camera_position_attrib_);
            // glesv2::enable_vertex_attrib_array(self.camera_tex_coord_attrib_);
            //
            // glesv2::draw_arrays(glesv2::GL_TRIANGLE_STRIP, 0, 4);
            //
            // glesv2::disable_vertex_attrib_array(self.camera_position_attrib_);
            // glesv2::disable_vertex_attrib_array(self.camera_tex_coord_attrib_);

            glesv2::use_program(0);
            glesv2::depth_mask(true);

            log::d("arcore::background_renderer::draw finish.\n");
        }
    }

    pub fn get_texture_id(&self) -> glesv2::GLuint {
        self.camera_texture_id_.clone()
    }
}