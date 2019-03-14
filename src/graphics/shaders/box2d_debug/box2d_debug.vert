#version 400

uniform Camera {
    mat4 u_camera;
};

in vec2 a_pos;
in vec4 a_color;

out vec4 v_color;

void main() {
    gl_Position = u_camera * vec4(a_pos, 0, 1);
    v_color = a_color;
}
