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

/// ArAnchor Color
struct ColoredAnchor {
  ArAnchor *anchor;
  float color[4];
};

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

//  bool show_plane;
//  bool show_point;
//  bool show_image;
//  bool show_faces;

  bool anchored;
  ArAnchor *anchor;
  float color[4];
//  mozilla::HashMap<int32_t, ColoredAnchor> plane_obj_map_;
//  mozilla::HashMap<int32_t, ColoredAnchor> image_obj_map_;
//  mozilla::HashMap<int32_t, ColoredAnchor> faces_obj_map_;

  float proj_mat4x4[16];
  float view_mat4x4[16];
  float mode_mat4x4[16];
};

extern "C" {

/// initial ArCore
void init_arcore(ArCore *arcore, JNIEnv *env);

/// on surface created
void on_surface_created(ArCore *arcore);

/// set display rotation, width, height
void on_display_changed(ArCore *arcore, int32_t rotation, int32_t width, int32_t height);

/// draw background and set relevant matrix
void on_draw_frame(ArCore *arcore);

/// touch to anchor
void on_touched(ArCore *arcore, float x, float y);

} // extern "C"

#endif