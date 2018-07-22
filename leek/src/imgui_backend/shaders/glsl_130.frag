#version 130

uniform sampler2D Texture;

in vec2 f_uv;
in vec4 f_color;

out vec4 out_color;

void main() {
  out_color =  f_color * texture(Texture, f_uv.st);
}
