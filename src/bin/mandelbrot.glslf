#version 330 core

out vec4 color;

uniform float height, width, zoom;
uniform vec2 center;
int max_iterations = 100;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec2 c = gl_FragCoord.xy / vec2(width, height) * 4.0 - 2.0;
    c = c / zoom - center;

    vec2 z = c;
    float i;
    for(i = 0; i < max_iterations; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        if(length(z) > 2.0) {
            break;
        }
    }

    if(i == max_iterations) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        float val = i / float(max_iterations);
        color = vec4(hsv2rgb(vec3(val, 1.0, 1.0)), 1.0);
    }
}
