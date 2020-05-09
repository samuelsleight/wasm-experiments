#version 300 es

precision mediump float;

in vec2 scene_position;

layout(std140) uniform global_uniforms {
    vec2 scene_dimensions;
};

layout(std140) uniform frame_uniforms {
    vec2 scene_offset;
    mediump float time;
};

void main() {
    vec2 device_position = 2.0 * (scene_position - scene_offset) / scene_dimensions - 1.0;
    gl_Position = vec4(device_position, 0.0, 1.0);
}
