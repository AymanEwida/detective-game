use std::ptr;

use gl::types::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Buffer {
    pub id: GLuint,
    target: GLuint
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);

        Self { id, target }
    }
}

impl Buffer {
    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(self.target, 0);
    }

    pub unsafe fn set_empty(&self, usage: GLuint) {
        gl::BufferData(
            self.target,
            1024,
            ptr::null(),
            usage
        );
    }

    pub unsafe fn set_data<V>(&self, vertices_data: &[V], usage: GLuint) {
        self.bind();
        
        let (_, data_bytes, _) = vertices_data.align_to::<u8>();

        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage
        );
    }

    pub unsafe fn set_sub_data<V>(&self, offset: usize, sub_data: &[V]) {
        self.bind();

        let offset = offset * std::mem::size_of::<V>();

        let (_, data_bytes, _) = sub_data.align_to::<u8>();

        gl::BufferSubData(
            self.target,
            offset as GLsizeiptr,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _
        );
    }
}
