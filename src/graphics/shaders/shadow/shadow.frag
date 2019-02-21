#version 400

out vec4 f_color;

void main() {
    f_color = vec4(0, 0, 0, 0.6);
    gl_FragDepth = gl_FragCoord.z;
}
