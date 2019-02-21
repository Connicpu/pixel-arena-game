#version 400

uniform sampler2DArray tex;

in vec2 v_uv;
flat in int v_imagelayer;

out vec4 f_color;

void main() {
    f_color = texture(tex, vec3(v_uv, v_imagelayer));
    if (f_color.a < 0.3) {
        gl_FragDepth = 0;
    } else {
        gl_FragDepth = gl_FragCoord.z;
    }
}
