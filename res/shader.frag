#version 330

layout(location=0) in vec2 v_texture;
layout(location=0) out vec4 f_color;

layout(set=0, binding=0) uniform texture2D t_texture;
layout(set=0, binding=1) uniform sampler s_texture;

void main(){
  //f_color = texture(sampler2D(t_texture, s_texture), v_texture);
  f_color = vec4(1.0, 1.0, 1.0, 1.0);
}

