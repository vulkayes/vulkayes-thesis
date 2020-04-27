#version 460

layout(location = 0) in vec4 color;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec4 out_color;

const vec3 LIGHT_DIRECTION = vec3(1.0, -1.0, 1.0);
const vec3 WHITE = vec3(1.0, 1.0, 1.0);

void main() {
	float brightness = dot(
		normalize(normal),
		normalize(LIGHT_DIRECTION)
	) / 4.0;

	out_color = vec4(
		mix(color.rgb, WHITE, brightness),
		color.a
	);
}
