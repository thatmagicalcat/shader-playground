#version 300 es

out vec3 v_color;

layout(location = 0) in vec3 pos;

void main() {
    gl_Position = vec4(pos, 1.0);
}
