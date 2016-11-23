#version 400

in vec3 position;
in vec2 texcoord;

uniform mat4 ProjMatrix;
uniform mat4 ModelMatrix;

out vec2 vTexcoord;

void main() {
    vTexcoord = texcoord;
    gl_Position = ProjMatrix * ModelMatrix * vec4(position, 1.0);
}