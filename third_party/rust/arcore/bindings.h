#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const int32_t K_NUM_VERTICES = 4;

/// ArCore
struct ArCore {
  int32_t width_;
  int32_t height_;
  int32_t rotation_;
  ArSession *ar_session;
  ArFrame *ar_frame;
  GLuint camera_program_;
  GLuint camera_position_attrib_;
  GLuint camera_tex_coord_attrib_;
  GLuint camera_texture_uniform_;
  GLuint camera_texture_id_;
  float uvs_transformed_[8];
  bool uvs_initialized_;
  float view_mat4x4[16];
  float proj_mat4x4[16];
};

static const GLenum GL_TEXTURE_EXTERNAL_OES = 36197;

extern "C" {

/// initial ArCore
void init_arcore(ArCore *arcore, JNIEnv *env);

/// on surface created
void on_surface_created(ArCore *arcore);

/// set display rotation, width, height
void on_display_changed(ArCore *arcore, int32_t rotation, int32_t width, int32_t height);

/// draw background and set relevant matrix
void on_draw_frame(ArCore *arcore);

/// get project matrix
float (get_proj_matrix(ArCore arcore))[16];

} // extern "C"
