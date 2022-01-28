#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const int32_t K_NUM_VERTICES = 4;

template<typename K = void, typename V = void, typename Hasher = void>
struct HashMap;

template<typename T = void>
struct Option;

struct BackgroundRenderer {
  GLuint shader_program_;
  GLuint texture_id_;
  GLuint attribute_vertices_;
  GLuint attribute_uvs_;
  GLuint uniform_texture_;
  float transformed_uvs_[8];
  bool uvs_initialized_;
};

struct ColoredAnchor {
  ArAnchor *anchor;
  float color[4];
};

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
  Option<BackgroundRenderer> renderer_background_;
  HashMap<int32_t, ColoredAnchor> plane_obj_map_;
  HashMap<int32_t, ColoredAnchor> point_obj_map_;
  HashMap<int32_t, ColoredAnchor> image_obj_map_;
  HashMap<int32_t, ColoredAnchor> faces_obj_map_;
  uintptr_t number_to_render;
  float view_mat4x4[16];
  float proj_mat4x4[16];
};

static const GLenum TEXTURE_EXTERNAL_OES = 36197;

extern "C" {

ArCore init_arcore();

void on_display_changed(ArCore arcore, int32_t display_rotation, int32_t width, int32_t height);

void on_draw(ArCore arcore);

float (get_proj_matrix(ArCore arcore))[16];

} // extern "C"
