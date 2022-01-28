use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::rc::Rc;
use libc::c_char;

use opengles::glesv2;

use crate::ffi_arcore::*;
use crate::log;
use crate::util;

pub const TEXTURE_EXTERNAL_OES: glesv2::GLenum = 0x8D65;

pub const K_NUM_VERTICES: i32 = 4;

const VS_SRC: &'static [u8] = b"
    attribute vec4 vertex;
    attribute vec2 textureCoords;
    varying vec2 v_textureCoords;
    void main() {
    v_textureCoords = textureCoords;
        gl_Position = vertex;
    }
\0";

const FS_SRC: &'static [u8] = b"
    #extension GL_OES_EGL_image_external : require
    precision mediump float;
    uniform samplerExternalOES texture;
    varying vec2 v_textureCoords;
    void main() {
      gl_FragColor = texture2D(texture, v_textureCoords);
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

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BackgroundRenderer {
    shader_program_: glesv2::GLuint,
    texture_id_: glesv2::GLuint,
    attribute_vertices_: glesv2::GLuint,
    attribute_uvs_: glesv2::GLuint,
    uniform_texture_: glesv2::GLuint,
    transformed_uvs_: [f32; 8],
    uvs_initialized_: bool,
}

impl BackgroundRenderer {
    pub fn new() -> BackgroundRenderer {
        log::i("arcore::background_renderer::new");
        unsafe {
            let shader_program = util::create_program(VS_SRC, FS_SRC);

            if shader_program == 0 {
                log::e("arcore::background_renderer::new Could not create program.");
            }

            let texture_id = glesv2::gen_textures(1)[0];
            glesv2::bind_texture(TEXTURE_EXTERNAL_OES, texture_id);
            glesv2::tex_parameteri(TEXTURE_EXTERNAL_OES, glesv2::GL_TEXTURE_MIN_FILTER, glesv2::GL_LINEAR as i32);
            glesv2::tex_parameteri(TEXTURE_EXTERNAL_OES, glesv2::GL_TEXTURE_MAG_FILTER, glesv2::GL_LINEAR as i32);


            let uniform_texture = glesv2::get_uniform_location(shader_program, "texture") as u32;
            let attribute_vertices = glesv2::get_attrib_location(shader_program, "vertex") as u32;
            let attribute_uvs = glesv2::get_attrib_location(shader_program, "textureCoords") as u32;

            let transformed_uvs: [f32; 8] = [0.0; 8];

//            log::d(&format!("arcore::background_renderer::new shader_program : {}", shader_program));
//            log::d(&format!("arcore::background_renderer::new texture_id : {:?}", texture_id));
//            log::d(&format!("arcore::background_renderer::new uniform_texture : {}", uniform_texture));
//            log::d(&format!("arcore::background_renderer::new attribute_vertices : {}", attribute_vertices));
//            log::d(&format!("arcore::background_renderer::new attribute_uvs : {}", attribute_uvs));

            BackgroundRenderer {
                shader_program_: shader_program,
                texture_id_: texture_id,
                attribute_vertices_: attribute_vertices,
                attribute_uvs_: attribute_uvs,
                uniform_texture_: uniform_texture,
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

            log::d(&format!("arcore::background_renderer::draw geometry_changed : {}", *geometry_changed));

            if *geometry_changed != 0 || !self.uvs_initialized_ {
                ArFrame_transformDisplayUvCoords(session, frame, K_NUM_VERTICES * 2, &K_UVS as *const f32, self.transformed_uvs_.as_mut_ptr());
                self.uvs_initialized_ = true;
                log::d(&format!("arcore::background_renderer::draw self.uvs_initialized_ : {}", self.uvs_initialized_));
            }

            glesv2::use_program(self.shader_program_);
            glesv2::depth_mask(false);

            glesv2::uniform1i(self.uniform_texture_ as i32, 1);
            glesv2::active_texture(glesv2::GL_TEXTURE1);
            glesv2::bind_texture(TEXTURE_EXTERNAL_OES, self.texture_id_);


            let vbo = glesv2::gen_buffers(2);

            glesv2::bind_buffer(glesv2::GL_ARRAY_BUFFER, vbo[0]);
            glesv2::buffer_data(glesv2::GL_ARRAY_BUFFER, &K_VERTICES, glesv2::GL_STATIC_DRAW);

            glesv2::enable_vertex_attrib_array(self.attribute_vertices_);
            glesv2::vertex_attrib_pointer(self.attribute_vertices_, 3, glesv2::GL_FLOAT, false, 0, &[0]);

            glesv2::bind_buffer(glesv2::GL_ARRAY_BUFFER, vbo[1]);
            glesv2::buffer_data(glesv2::GL_ARRAY_BUFFER, &self.transformed_uvs_, glesv2::GL_STATIC_DRAW);

            glesv2::enable_vertex_attrib_array(self.attribute_uvs_);
            glesv2::vertex_attrib_pointer(self.attribute_uvs_, 2, glesv2::GL_FLOAT, false, 0, &[0]);

            glesv2::draw_arrays(glesv2::GL_TRIANGLE_STRIP, 0, 4);

            glesv2::use_program(0);
            glesv2::depth_mask(true);
        }
    }

    pub fn get_texture_id(&self) -> glesv2::GLuint {
        self.texture_id_.clone()
    }
}