#version 450

layout(location=0) in vec3 in_position;
layout(location=1) in vec2 in_texture;

layout(location=0) out vec2 v_texture;

void main() {
  v_texture = in_texture;

  gl_Position = vec4(in_position, 1.0);
}

