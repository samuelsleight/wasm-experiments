#version 300 es

precision mediump float;

layout(std140) uniform frame_uniforms {
    vec2 scene_offset;
    float time;
};

out vec4 colour;

void main() {
    colour = vec4(time / 1000.0, 1.0 - (time / 1000.0), 1.0, 1.0);
}
