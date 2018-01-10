extern crate egl;
extern crate libc;

use std::ptr;

pub struct GLPlatformContext {
    pub display: egl::egl::EGLDisplay,
    pub egl_context: egl::egl::EGLContext,
    egl_surface: egl::egl::EGLSurface,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        self.drop_current_context();
        egl::egl::DestroyContext(self.display, self.egl_context);
        egl::egl::DestroySurface(self.display, self.egl_surface);
    }
}

impl GLPlatformContext {
    pub fn new(width: i32, height: i32)
               -> Option<GLPlatformContext> {
        let config_attributes = [
            egl::egl::EGL_SURFACE_TYPE as i32, egl::egl::EGL_PBUFFER_BIT as i32,
            egl::egl::EGL_RENDERABLE_TYPE as i32, egl::egl::EGL_OPENGL_ES2_BIT as i32,
            egl::egl::EGL_RED_SIZE as i32, 8,
            egl::egl::EGL_BLUE_SIZE as i32, 8,
            egl::egl::EGL_ALPHA_SIZE as i32, 8,
            egl::egl::EGL_NONE as i32,
        ];

        let display = egl::egl::GetDisplay(egl::egl::EGL_DEFAULT_DISPLAY as *mut libc::c_void);
        let mut surface_config = ptr::null_mut();
        let mut number_of_configs = 0;
        egl::egl::ChooseConfig(display,
                          config_attributes.as_ptr(),
                          &mut surface_config, 1, &mut number_of_configs);
        if number_of_configs == 0 {
            return None;
        }

        let context_attributes = [
            egl::egl::EGL_CONTEXT_CLIENT_VERSION as i32, 2,
            egl::egl::EGL_NONE as i32
        ];
        let egl_context = egl::egl::CreateContext(display,
                                             surface_config,
                                             egl::egl::EGL_NO_CONTEXT as egl::egl::EGLContext,
                                             context_attributes.as_ptr());
        if egl_context == egl::egl::EGL_NO_CONTEXT as egl::egl::EGLContext {
            return None;
        }

        let mut surface_attributes = [
            egl::egl::EGL_WIDTH as i32, width,
            egl::egl::EGL_HEIGHT as i32, height,
            egl::egl::EGL_NONE as i32,
        ];
        let egl_surface = egl::egl::CreatePbufferSurface(display,
                                                    surface_config,
                                                    &mut surface_attributes[0]);
        if egl_surface == egl::egl::EGL_NO_SURFACE as egl::egl::EGLSurface {
            egl::egl::DestroyContext(display, egl_context);
            return None;
        }


        Some(GLPlatformContext {
            display,
            egl_context,
            egl_surface,
        })
    }

    pub fn drop_current_context(&self) {
        egl::egl::MakeCurrent(self.display, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    }

    pub fn make_current(&self) {
        egl::egl::MakeCurrent(self.display, self.egl_surface, self.egl_surface, self.egl_context);
    }
}