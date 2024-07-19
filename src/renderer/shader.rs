use std::{ffi::CString, ptr};

use gl::types::*;

use super::error::{Result, Error};

pub struct Shader {
    pub id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self> {
        let source_code = CString::new(source_code)
            .map_err(|_| Error::CompilationError("Unable to create shader source code".to_string()))?;

        let shader = Self {
            id: gl::CreateShader(shader_type),
        };

        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            return Ok(shader);
        }

        let mut error_log_size: GLint = 0;
        gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);

        let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
        gl::GetShaderInfoLog(
            shader.id, 
            error_log_size, 
            &mut error_log_size, 
            error_log.as_mut_ptr() as *mut _
        );

        error_log.set_len(error_log_size as usize);
        let log = String::from_utf8(error_log)
            .map_err(|_| Error::CompilationError("Unable to parse error log to string".to_string()))?;

        Err(Error::CompilationError(log))
    }    
}
