#version 400

uniform Camera {
    mat4 u_camera;
};

uniform vec2 chunk_pos;
uniform vec2 tile_size;
uniform float layer;

uniform sampler1D rect_buffer;

in vec2 a_pos;
in vec2 a_uv;

in vec2 tile_pos;
in int tile_id;

out vec2 v_uv;

void main() {
    vec4 uvrect = texelFetch(rect_buffer, tile_id, 0);
    vec2 pos = chunk_pos + (tile_pos * vec2(1, -1) + a_pos) * tile_size;

    gl_Position = u_camera * vec4(pos, layer, 1);
    v_uv = mix(uvrect.xy, uvrect.zw, a_uv);
}


