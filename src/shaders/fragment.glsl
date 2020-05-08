precision mediump float;
uniform float time;

void main() {
    gl_FragColor = vec4(time / 1000.0, 1.0 - (time / 1000.0), 1.0, 1.0);
}
