#version 400
in vec2 vTexcoord;
in vec4 vColor;
uniform sampler2D diffuseTexture;
out vec4 out_color;
void main() {
    vec4 diffuse = texture2D(diffuseTexture, vTexcoord);
    out_color = diffuse * vColor;
}