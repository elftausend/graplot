use crate::gl::{GLenum, GLint, GLuint, GLuint64};

pub const GL_TIME_ELAPSED: u32 = 35007;

pub unsafe fn glGetQueryObjectui64v(_id: GLuint, _pname: GLenum, _params: *mut GLuint64) {
    unimplemented!();
}

pub unsafe fn glGetQueryObjectiv(_id: GLuint, _pname: GLenum, _params: *mut GLint) {
    unimplemented!();
}

pub unsafe fn sapp_is_elapsed_timer_supported() -> bool {
    return false;
}

