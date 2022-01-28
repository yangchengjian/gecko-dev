#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#include "glm/glm.hpp"
#include "arcore/arcore_c_api.h"

static const int32_t K_NUM_VERTICES = 4;

template<typename K = void, typename V = void, typename Hasher = void>
struct HashMap;

struct BackgroundRenderer {
  GLuint shader_program_;
  GLuint texture_id_;
  GLuint attribute_vertices_;
  GLuint attribute_uvs_;
  GLuint uniform_texture_;
  float transformed_uvs_[8];
  bool uvs_initialized_;
};

struct PointCloudRenderer {
  GLuint shader_program_;
  GLuint attribute_vertices_;
  GLuint uniform_mvp_mat_;
};

//struct PlaneRenderer {
//  Vec<Vec3> vertices_;
//  Vec<GLushort> triangles_;
//  float model_mat_[16];
//  Vec3 normal_vec_;
//  GLuint texture_id_;
//  GLuint shader_program_;
//  GLuint attri_vertices_;
//  GLuint uniform_mvp_mat_;
//  GLuint uniform_texture_;
//  GLuint uniform_model_mat_;
//  GLuint uniform_normal_vec_;
//  GLuint uniform_color_;
//};

struct ArCore {
  ArSession *ar_session;
  ArFrame *ar_frame;
  bool show_plane;
  bool show_point;
  bool show_image;
  bool show_faces;
  int32_t shop_rate;
  int32_t width_;
  int32_t height_;
  int32_t display_rotation_;
  GLuint background_texture_id;
  BackgroundRenderer renderer_background_;
  PointCloudRenderer renderer_point_cloud_;
//  PlaneRenderer renderer_plane_;
  uintptr_t number_to_render;
  float view_mat4x4[16];
  float proj_mat4x4[16];
};

extern "C" {

ArCore init_arcore();

void on_display_changed(ArCore arcore,
                        int32_t display_rotation,
                        int32_t width,
                        int32_t height);

void on_draw(ArCore arcore);

float *get_proj_matrix(ArCore arcore);

} // extern "C"
