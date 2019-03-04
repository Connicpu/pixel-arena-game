#version 400

uniform sampler2D tex;

in vec2 v_uv;

out vec4 f_color;

void main() {
    f_color = texture(tex, v_uv);
    if (f_color.a < 0.3) {
        gl_FragDepth = 0;
    } else {
        gl_FragDepth = gl_FragCoord.z;
    }
}
