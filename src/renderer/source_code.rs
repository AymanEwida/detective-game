use crate::library::utils::template_literal;

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

pub enum SourceCodeType {
    Vertex,
    Fragment
}

pub struct SourceCode {
    source_code: String
}

impl SourceCode {
    pub fn new(soruce_type: SourceCodeType, source_code: &str, size: Option<Size>) -> Self {
        match soruce_type {
            SourceCodeType::Fragment => {
                Self {
                    source_code: source_code.to_string(),
                }
            },
            _ => {
                if let Some((width, height)) = size {
                    let width_string = &(width as f32/2.0).to_string();
                    let height_string = &(height as f32).to_string();
            
                    return Self {
                        source_code: template_literal(source_code, Some(&[width_string, height_string])),
                    };
                } else {
                    return Self {
                        source_code: source_code.to_string(),
                    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertices_vertex_source_code() {
        let vertices_vertex_source_code = SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some((5, 1)));
        let actual = vertices_vertex_source_code.get_source_code();
        let expected = "\n#version 330\nin vec2 position;\nin vec3 color;\nout vec3 vertexColor;\nvoid main() {\n    gl_Position = vec4(position, 2.5, 1);\n    vertexColor = color;\n}\n";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vertices_fragment_source_code() {
        let vertices_fragment_source_code = SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None);
        let actual = vertices_fragment_source_code.get_source_code();
        let expected = "\n#version 330\nout vec4 FragColor;\nin vec3 vertexColor;\nvoid main() {\n    FragColor = vec4(vertexColor, 1.0);\n}\n";

        assert_eq!(actual, expected);
    }
}
