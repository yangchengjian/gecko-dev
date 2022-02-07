#ifndef ARCORE_BINDINGS_H_
#define ARCORE_BINDINGS_H_

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <type_traits>

#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>

#include "arcore_c_api.h"
#include "GLContext.h"
#include "glm/glm.hpp"

/// ArCore
struct ArCore {
  int32_t width_;
  int32_t height_;
  int32_t rotation_;

  ArSession *ar_session;
  ArFrame *ar_frame;

  GLuint camera_program_;
  GLuint camera_texture_id_;
  GLuint camera_position_attrib_;
  GLuint camera_tex_coord_attrib_;
  GLuint camera_texture_uniform_;
  float uvs_transformed_[8];
  bool uvs_initialized_;

  float view_mat4x4[16];
  float proj_mat4x4[16];
};

extern "C" {

/// initial ArCore
void init_arcore(ArCore *arcore, void *env);

/// on surface created
void on_surface_created(ArCore *arcore);

/// set display rotation, width, height
void on_display_changed(ArCore *arcore, int32_t display_rotation, int32_t width, int32_t height);

/// draw background and set relevant matrix
void on_draw_frame(ArCore *arcore);


void after_init_arcore(ArCore *arcore, mozilla::gl::GLContext* gl);

void after_draw_frame(ArCore *arcore, mozilla::gl::GLContext* gl);

} // extern "C"

#endif