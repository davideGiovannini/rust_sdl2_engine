#version 130
in vec2 f_uv;
out vec4 LFragment;

uniform sampler2D framebuffer;

void main() {
     LFragment = mix(texture(framebuffer, f_uv), vec4(f_uv, 0.0, 1.0), 0.5);
}
