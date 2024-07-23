use detective_game::renderer::source_code::{SourceCode, SourceCodeType, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE};

#[test]
fn test_vertices_vertex_source_code() {
    let vertices_vertex_source_code = SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(1));
    let actual = vertices_vertex_source_code.get_source_code();
    let expected = "\n#version 330\nin vec2 position;\nin vec3 color;\nout vec3 vertexColor;\nvoid main() {\n    gl_Position = vec4(position, 0.0, 1);\n    vertexColor = color;\n}\n";

    assert_eq!(actual, expected);
}

#[test]
fn test_vertices_fragment_source_code() {
    let vertices_fragment_source_code = SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None);
    let actual = vertices_fragment_source_code.get_source_code();
    let expected = "\n#version 330\nout vec4 FragColor;\nin vec3 vertexColor;\nvoid main() {\n    FragColor = vec4(vertexColor, 1.0);\n}\n";

    assert_eq!(actual, expected);
}