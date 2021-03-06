#version 400

in vec3 position;
in vec2 texcoord;
in vec4 color;

uniform mat4 ProjMatrix;
uniform mat4 ModelMatrix;

out vec2 vTexcoord;
out vec4 vColor;

void main() {
    vTexcoord = texcoord;
    vColor = color;
    gl_Position = ProjMatrix * ModelMatrix * vec4(position, 1.0);
}