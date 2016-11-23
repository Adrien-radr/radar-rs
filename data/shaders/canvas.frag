#version 400
in vec2 vTexcoord;

uniform vec4 backColor;
uniform vec2 canvasPosition;
uniform vec2 canvasSize;

out vec4 out_color;

vec2 cpx_sq( vec2 a ) {
    return vec2(a.x*a.x - a.y*a.y, 2.0*a.x*a.y);
}

void main() {
    // vec2 v(0.0);
    // for(int i =0;i < 256; ++i)
        // v 
    out_color = backColor * vec4(vTexcoord.x, vTexcoord.y, 1, 1);
}