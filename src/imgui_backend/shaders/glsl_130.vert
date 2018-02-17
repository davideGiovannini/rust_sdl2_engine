#version 130

uniform mat4 ProjMtx;

in vec2 Position;
in vec2 UV;
in vec4 Color;

out vec2 f_uv;
out vec4 f_color;

// Built-in:
// vec4 gl_Position

void main() {
  f_uv = UV;
  f_color = Color;
  gl_Position = ProjMtx * vec4(Position.xy, 0, 1);
}
