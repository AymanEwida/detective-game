use std::path::Path;

use gl::types::*;
use image::EncodableLayout;

use super::error::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Texture {
    pub id: GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl Texture {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);

        Self { id }
    }
}

impl Texture {
    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    pub unsafe fn load(&self, path: &Path) -> Result<()> {
        self.bind();

        let image = image::open(path)
            .map_err(|_| Error::LoadImageError("Unable to get image in RGBA format".to_string()))
            ?.into_rgba8();

        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGBA as i32, 
            image.width() as i32, 
            image.height() as i32, 
            0, 
            gl::RGBA, 
            gl::UNSIGNED_BYTE, 
            image.as_bytes().as_ptr() as *const _
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        Ok(())
    }

    pub unsafe fn set_wrapping(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as GLint);
    }

    pub unsafe fn set_filtering(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode as GLint);
    }
}
