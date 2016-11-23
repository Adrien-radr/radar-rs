#version 400

in vec3 position;
in vec2 texcoord;
in vec4 color;
out vec2 vTexcoord;
out vec4 vColor;
void main() {
    vTexcoord = texcoord;
    vColor = color;
    gl_Position = vec4(position, 1.0);
}