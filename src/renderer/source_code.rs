use super::render::Size;

pub const VERTICES_VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec3 color;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(position, {}, {});
    vertexColor = color;
}
"#;

pub const VERTICES_FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec3 vertexColor;
void main() {
    FragColor = vec4(vertexColor, 1.0);
}
"#;

pub struct SourceCode<'a> {
    source_code: &'a str
}

impl<'a> SourceCode<'a> {
    fn new(source_code: &'a str, (width, height): Size) -> Self {
        Self {
            source_code
        }
    }
}
