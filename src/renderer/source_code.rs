use crate::library::utils::template_literal;

pub const VERTICES_VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec4 color;
out vec4 vertexColor;
void main() {
    gl_Position = vec4(position, 0.0, {});
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
    gl_Position = vec4(position, 0.0, {});
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

pub enum SourceCodeType {
    Vertex,
    Fragment
}

pub struct SourceCode {
    source_code: String
}

impl SourceCode {
    pub fn new(soruce_type: SourceCodeType, source_code: &str, height: Option<usize>) -> Self {
        match soruce_type {
            SourceCodeType::Fragment => {
                Self {
                    source_code: source_code.to_string(),
                }
            },
            _ => {
                match height {
                    Some(height) => {
                        let height_string = &height.to_string();
                
                        Self {
                            source_code: template_literal(source_code, Some(&[height_string])),
                        }
                    },
                    None => {
                        Self {
                            source_code: source_code.to_string(),
                        }
                    }
                }
            }
        }
    }
}

impl SourceCode {
    pub fn get_source_code(&self)-> &str {
        &self.source_code
    }
}
