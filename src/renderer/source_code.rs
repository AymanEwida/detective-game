pub const VERTICES_VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec4 color;
out vec4 vertexColor;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vertexColor = color;
}
"#;

pub const VERTICES_FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec4 vertexColor;
void main() {
    FragColor = vertexColor;
}
"#;

pub const TEXTURE_VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec2 vertexTexCoord;
out vec2 texCoord;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    texCoord = vertexTexCoord;
}
"#;

pub const TEXTURE_FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec2 texCoord;
uniform sampler2D texture0;
void main() {
    FragColor = texture(texture0, texCoord);
}
"#;
