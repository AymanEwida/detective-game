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
in vec4 vertexColor;

out vec4 FragColor;

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
in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D texture0;
uniform vec3 textColor;
uniform int isText;

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(texture0, texCoord).r);
    
    FragColor = vec4(textColor, 1.0) * sampled * isText + texture(texture0, texCoord) * (1- isText);
}
"#;
