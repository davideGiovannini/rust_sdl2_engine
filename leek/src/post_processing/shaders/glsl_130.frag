#version 130
in vec2 f_uv;
out vec4 LFragment;

uniform sampler2D framebuffer;

void main() {
    float len = 1.0 - 1.5 * length(vec2((f_uv.x - 0.5), (f_uv.y - 0.5)));
    vec4 color = len * texture(framebuffer, f_uv);
    LFragment =  vec4(color.xyz, 1.0);//mix(texture(framebuffer, f_uv) , vec4(len ,len, len, 1.0), 0.9);
}
