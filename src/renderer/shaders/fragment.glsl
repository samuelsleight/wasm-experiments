#version 300 es

precision mediump float;

layout(std140) uniform frame_uniforms {
    vec2 scene_offset;
    float time;
};

uniform sampler2D tex;

in vec2 fragment_tex_coords;

out vec4 colour;

void main() {
    colour = texture(tex, fragment_tex_coords);
}
