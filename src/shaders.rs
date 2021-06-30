pub const VERTEX_SHADER: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 tex_pos;

uniform vec2 size;
uniform bool is_percentage;
  
out vec4 vertex_color;
out vec2 vertex_tex_pos;

void main()
{
    float x, y;
    if (!is_percentage) {
        x = position.x / size.x;
        y = position.y / size.y;
        x = x * 2 - 1;
        y = 1 - y * 2;
    } else {
        x = position.x * 2 - 1;
        y = 1 - position.y * 2;
    }
    gl_Position = vec4(x, y, 1.0, 1.0);
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
        FragColor = texture(tex, vertex_tex_pos) * vertex_color;
    } else {
        FragColor = vertex_color;
    }
} 
"#;
