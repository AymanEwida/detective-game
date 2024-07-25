use detective_game::renderer::source_code::*;

#[test]
fn test_vertices_vertex_source_code() {
    let vertices_vertex_source_code = SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(1));
    let actual = vertices_vertex_source_code.get_source_code();
    let expected = "\n#version 330\nin vec2 position;\nin vec4 color;\nout vec4 vertexColor;\nvoid main() {\n    gl_Position = vec4(position, 0.0, 1);\n    vertexColor = color;\n}\n";

    assert_eq!(actual, expected);
}

#[test]
fn test_vertices_fragment_source_code() {
    let vertices_fragment_source_code = SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None);
    let actual = vertices_fragment_source_code.get_source_code();
    let expected = "\n#version 330\nout vec4 FragColor;\nin vec4 vertexColor;\nvoid main() {\n    FragColor = vertexColor;\n}\n";

    assert_eq!(actual, expected);
}

#[test]
fn test_texture_vertex_source_code() {
    let vertices_vertex_source_code = SourceCode::new(SourceCodeType::Vertex, TEXTURE_VERTEX_SHADER_SOURCE, Some(5));
    let actual = vertices_vertex_source_code.get_source_code();
    let expected = "\n#version 330\nin vec2 position;\nin vec2 vertexTexCoord;\nout vec2 texCoord;\nvoid main() {\n    gl_Position = vec4(position, 0.0, 5);\n    texCoord = vertexTexCoord;\n}\n";

    assert_eq!(actual, expected);
}

#[test]
fn test_texture_fragment_source_code() {
    let vertices_fragment_source_code = SourceCode::new(SourceCodeType::Fragment, TEXTURE_FRAGMENT_SHADER_SOURCE, None);
    let actual = vertices_fragment_source_code.get_source_code();
    let expected = "\n#version 330\nout vec4 FragColor;\nin vec2 texCoord;\nuniform sampler2D texture0;\nvoid main() {\n    FragColor = texture(texture0, texCoord);\n}\n";

    assert_eq!(actual, expected);
}
