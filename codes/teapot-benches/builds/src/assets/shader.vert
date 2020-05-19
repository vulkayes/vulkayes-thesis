#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec4 out_color;
layout(location = 1) out vec3 out_normal;

layout(set = 0, binding = 0) uniform FrameState {
	mat4 view_matrix;
	mat4 projection_matrix;
} frame_state;

layout(push_constant) uniform ObjectState {
	mat4 world_matrix;
	vec4 color;
} object_state;

void main() {
	mat4 world_view = frame_state.view_matrix * object_state.world_matrix;

	gl_Position = frame_state.projection_matrix * world_view * vec4(position, 1.0);
	out_color = object_state.color;
	out_normal = transpose(
		inverse(mat3(world_view))
	) * normal;
}
