#version 450
out vec4 out_color;

in vec3 ourTexcoords;

uniform sampler2D texture0;
uniform vec2 penis;

void main() {
  out_color= texture(texture0, vec2(ourTexcoords.x, ourTexcoords.y));
  //out_color = ourColor;
}
