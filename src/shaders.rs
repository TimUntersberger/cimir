pub const VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 tex_pos;

uniform mat4 projection;
  
out vec4 vertex_color;
out vec2 vertex_tex_pos;

void main()
{
    gl_Position = projection * vec4(position, 0.0, 1.0);
    vertex_color = vec4(color, 1.0);
    vertex_tex_pos = tex_pos;
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
#version 330 core
out vec4 FragColor;

uniform sampler2D tex;
uniform bool use_texture;
  
in vec4 vertex_color;
in vec2 vertex_tex_pos;

void main()
{
    if (use_texture) {
        FragColor = texture(tex, vertex_tex_pos);
    } else {
        FragColor = vertex_color;
    }
} 
"#;

pub const FONT_VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 tex_pos;

uniform mat4 projection;
  
out vec4 vertex_color;
out vec2 vertex_tex_pos;

void main()
{
    gl_Position = projection * vec4(position, 0.0, 1.0);
    vertex_color = vec4(color, 1.0);
    vertex_tex_pos = tex_pos;
}
"#;

pub const FONT_FRAGMENT_SHADER: &'static str = r#"
#version 330 core
out vec4 FragColor;

uniform sampler2D tex;
  
in vec4 vertex_color;
in vec2 vertex_tex_pos;

void main()
{
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(tex, vertex_tex_pos).r);
    FragColor = vertex_color * sampled;
} 
"#;
