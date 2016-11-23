#version 400
in vec2 vTexcoord;

uniform vec4 backColor;

out vec4 out_color;

void main() {
    // vec4 diffuse = texture2D(diffuseTexture, vTexcoord);
    out_color = backColor;
}