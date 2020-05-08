#version 300 es

precision mediump float;
uniform float time;

out vec4 colour;

void main() {
    colour = vec4(time / 1000.0, 1.0 - (time / 1000.0), 1.0, 1.0);
}
