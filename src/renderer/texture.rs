use std::path::Path;

use gl::types::*;
use image::EncodableLayout;

use super::{error::{Error, Result}, render::Size};

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

    pub unsafe fn load_image(&self, path: &Path) -> Result<()> {
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

    pub unsafe fn load_bytes(&self, size: Size, bytes: Vec<u8>) {
        self.bind();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as GLint,
            size.width as GLsizei,
            size.height as GLsizei,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            bytes.as_ptr() as *const _,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
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
