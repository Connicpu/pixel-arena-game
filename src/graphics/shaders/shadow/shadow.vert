#version 400

uniform Camera {
    mat4 u_camera;
};

in vec2 a_pos;
in vec2 pos;
in float size;
in float z;

void main() {
    gl_Position = u_camera * vec4(pos + a_pos * vec2(0.4, 0.2625) * size, z - 0.05, 1);
}
