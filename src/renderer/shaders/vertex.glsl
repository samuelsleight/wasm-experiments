#version 300 es

in vec2 scene_position;

uniform vec2 scene_offset;
uniform vec2 scene_dimensions;

void main() {
    vec2 device_position = 2.0 * (scene_position - scene_offset) / scene_dimensions - 1.0;
    gl_Position = vec4(device_position, 0.0, 1.0);
}
