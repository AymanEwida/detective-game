use std::ffi::CString;

use gl::types::*;

use super::{error::{Error, Result}, shader::Shader};

pub struct Program {
    id: GLuint,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Program {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            return Ok(program);
        }

        let mut error_log_size: GLint = 0;
        gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);

        let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
        gl::GetProgramInfoLog(
            program.id, 
            error_log_size, 
            &mut error_log_size, 
            error_log.as_mut_ptr() as *mut _
        );

        error_log.set_len(error_log_size as usize);
        let log = String::from_utf8(error_log)
            .map_err(|_| Error::LinkingError("Unable to parse error log to string".to_string()))?;

        Err(Error::LinkingError(log))
    }
}

impl Program {
    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint> {
        let attrib = CString::new(attrib)
            .map_err(|_| Error::AttributeLocationError("Unable to get attrib as CString".to_string()))?;
        
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }

    pub unsafe fn set_int_uniform(&self, name: &str, value: i32) -> Result<()> {
        self.apply();

        let uniform = CString::new(name)
            .map_err(|_| Error::LinkingError("Unable to get uniform string".to_string()))?;

        gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
        
        Ok(())
    }

    pub unsafe fn set_color_data_uniform(&self, name: &str, data: (f32, f32, f32)) -> Result<()> {
        let uniform = CString::new(name)
            .map_err(|_| Error::LinkingError("Unable to get uniform string".to_string()))?;

        gl::Uniform3f(gl::GetUniformLocation(self.id, uniform.as_ptr()), data.0, data.1, data.2);

        Ok(())
    }

    pub unsafe fn set_bool_uniform(&self, name: &str, value: i32) -> Result<()> {
        assert!(value == 1 || value == 0, "value must be 1 (true) or 0 (false)");

        let uniform = CString::new(name)
            .map_err(|_| Error::LinkingError("Unable to get uniform string".to_string()))?;

        gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);

        Ok(())
    }
}
