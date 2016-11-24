#version 400
in vec2 vTexcoord;

uniform vec4 backColor;
uniform vec2 canvasPosition;
uniform vec2 canvasSize;

out vec4 out_color;

const float min_x = 1.2/4.0;
const float min_y = 1.2/4.0;
const float scale_x = 1.0/12.0;
const float scale_y = 1.0/12.0;

vec2 cpx_sq( vec2 a ) {
    return vec2(a.x*a.x - a.y*a.y, 2.0*a.x*a.y);
}
vec2 cpx_add( vec2 a, vec2 b ) {
    return vec2(a.x+b.x, a.y+b.y);
}

vec2 mandelbrot( vec2 v ) {
    return cpx_add( cpx_sq(v), vec2(min_x + ((vTexcoord.x*2.0)) * scale_x, min_y + (vTexcoord.y*1.0) * scale_y) );
}

vec2 burningShip( vec2 v ) {
    vec2 c = vec2(-2 + ((vTexcoord.x*3.0)), -1 + (vTexcoord.y*2.0));
    vec2 vsq = cpx_sq( abs( v ) );
    return cpx_add( vsq, c );
}

void main() {
    vec2 v = vec2(0.0, 0.0);
    const int max_iter = 256;
    int quit_iter = max_iter;
    for(int i = 0; i < max_iter; ++i) {
        // v = mandelbrot(v);
        v = burningShip(v);

        if(dot(v,v) > 2.0) {
            quit_iter = i;
            break;
        }
    }

    float max_val = 20.0;
    vec3 frac_col;
    quit_iter = max_iter - quit_iter;
    if( quit_iter == 0 )
        frac_col = vec3(0.0, 0.0, 0.0);
    else {
        float u = quit_iter / float(max_iter);
        float val = log(max_val);
        frac_col = vec3(val, val, val);
    }

    out_color = backColor * vec4(frac_col, 1);
}