use sparkle::gl;

use crate::ffi_arcore::*;
use crate::log;
use crate::util;

const VS_SRC: &'static [u8] = b"
    attribute vec4 vertex;
    uniform mat4 mvp;
    void main() {
      gl_PointSize = 5.0;
      // Pointcloud vertex's w component is confidence value.
      // Not used in renderer.
      gl_Position = mvp * vec4(vertex.xyz, 1.0);
    }
\0";

const FS_SRC: &'static [u8] = b"
   precision lowp float;
    void main() {
      gl_FragColor = vec4(0.1215, 0.7372, 0.8235, 1.0);
    }
\0";

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PointCloudRenderer {
    shader_program_: gl::types::GLuint,
    attribute_vertices_: gl::types::GLuint,
    uniform_mvp_mat_: gl::types::GLuint,
}

impl PointCloudRenderer {
    pub fn new(gl: &gl::Gl) -> PointCloudRenderer {
        log::i("arcore::point_cloud_renderer::new");

        let shader_program = util::create_program(gl, VS_SRC, FS_SRC);
        if shader_program == 0 {
            log::e("arcore::point_cloud_renderer::new Could not create program.");
        }

        let attribute_vertices_ = gl.get_attrib_location(shader_program, "vertex") as u32;
        let uniform_mvp_mat_ = gl.get_uniform_location(shader_program, "mvp") as u32;

//        log::d(&format!("arcore::point_cloud_renderer::new shader_program : {}", shader_program));
//        log::d(&format!("arcore::point_cloud_renderer::new attribute_vertices_ : {}", attribute_vertices_));
//        log::d(&format!("arcore::point_cloud_renderer::new uniform_mvp_mat_ : {}", uniform_mvp_mat_));


        PointCloudRenderer {
            shader_program_: shader_program,
            attribute_vertices_: attribute_vertices_,
            uniform_mvp_mat_: uniform_mvp_mat_,
        }
    }

    pub fn draw(&mut self, gl: &gl::Gl, mvp_matrix: ::glm::Mat4, ar_session: *mut ArSession, ar_point_cloud: *mut ArPointCloud) {
        log::d("arcore::point_cloud_renderer::draw");

//        log::d(&format!("arcore::point_cloud_renderer::draw self.shader_program_ : {}", self.shader_program_));
//        log::d(&format!("arcore::point_cloud_renderer::draw self.attribute_vertices_ : {}", self.attribute_vertices_));
//        log::d(&format!("arcore::point_cloud_renderer::draw self.uniform_mvp_mat_ : {}", self.uniform_mvp_mat_));

        unsafe {
            gl.use_program(self.shader_program_);

            let mut number_of_points: usize = 0;

            ArPointCloud_getNumberOfPoints(ar_session as *const ArSession,
                                           ar_point_cloud as *const ArPointCloud,
                                           &mut number_of_points as *mut usize as *mut i32);


//            log::d(&format!("arcore::point_cloud_renderer::draw number_of_points : {:}", number_of_points));

            if number_of_points <= 0 {
                return;
            }

            let mut point_cloud_data: *const f32 = 0 as *const f32;
            ArPointCloud_getData(ar_session as *const ArSession,
                                 ar_point_cloud as *const ArPointCloud,
                                 &mut point_cloud_data);


            let mut point_cloud_vertexs = ::std::slice::from_raw_parts(point_cloud_data, number_of_points * 4);

            let mvp_array = util::get_array_from_mat4(mvp_matrix);

//            log::d(&format!("arcore::point_cloud_renderer::draw mvp_array : {:?}", mvp_array));
//            log::d(&format!("arcore::point_cloud_renderer::draw point_cloud_vertexs : {:?}", point_cloud_vertexs));

            gl.uniform_matrix_4fv(self.uniform_mvp_mat_ as i32, false, &mvp_array);

            let vbo = gl.gen_buffers(1);

            gl.bind_buffer(gl::ARRAY_BUFFER, vbo[0]);
            gl::buffer_data(gl, gl::ARRAY_BUFFER, &mut point_cloud_vertexs, gl::STATIC_DRAW);

            gl.enable_vertex_attrib_array(self.attribute_vertices_);
            gl.vertex_attrib_pointer(self.attribute_vertices_, 4, gl::FLOAT, false, 0, 0);

            gl.draw_arrays(gl::POINTS, 0, number_of_points as i32);

            gl.use_program(0);
        }
    }
}