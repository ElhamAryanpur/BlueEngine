#version 450

layout(location=0) out vec4 outColor;

void main(){
  //f_color = texture(sampler2D(t_texture, s_texture), v_texture);
  outColor = vec4(1.0, 1.0, 1.0, 1.0);
}

