#version 330
layout(location = 0) in vec3 position;
// layout(location = 1) in vec2 color;

out vec4 ourColor;

uniform mat4 view;
uniform mat4 proj;
uniform mat4 model;

void main() {
  // ourColor = vec4(color, 1.0, 1.0);
  gl_Position = proj * view * model * vec4(position, 1.0);
  ourColor = vec4(position, 1.0);
}
