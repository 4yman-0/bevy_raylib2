use raylib::core::error::Error;
use raylib::prelude::*;

pub struct RaylibContext {
    pub rl: RaylibHandle,
    pub(crate) thread: RaylibThread,
}

impl RaylibContext {
    pub fn begin_drawing(&mut self) -> RaylibDrawHandle {
        self.rl.begin_drawing(&self.thread)
    }

    pub fn draw(&mut self, func: impl FnMut(RaylibDrawHandle<'_>)) {
        self.rl.draw(&self.thread, func)
    }

    pub fn load_image_from_screen(&self) -> Image {
        self.rl.load_image_from_screen(&self.thread)
    }

    pub fn take_screenshot(&mut self, filename: &str) {
        self.rl.take_screenshot(&self.thread, filename)
    }

    pub fn load_model(&mut self, filename: &str) -> Result<Model, Error> {
        self.rl.load_model(&self.thread, filename)
    }

    pub fn load_model_from_mesh(&mut self, mesh: WeakMesh) -> Result<Model, Error> {
        self.rl.load_model_from_mesh(&self.thread, mesh)
    }

    pub fn load_model_animations(&mut self, filename: &str) -> Result<Vec<ModelAnimation>, Error> {
        self.rl.load_model_animations(&self.thread, filename)
    }

    pub fn update_model_animation(
        &mut self,
        model: impl AsMut<raylib::ffi::Model>,
        anim: impl AsRef<raylib::ffi::ModelAnimation>,
        frame: i32,
    ) {
        self.rl
            .update_model_animation(&self.thread, model, anim, frame)
    }

    pub fn update_model_animation_bones(
        &mut self,
        model: impl AsMut<raylib::ffi::Model>,
        anim: impl AsRef<raylib::ffi::ModelAnimation>,
        frame: i32,
    ) {
        self.rl
            .update_model_animation_bones(&self.thread, model, anim, frame)
    }

    pub fn load_material_default(&self) -> WeakMaterial {
        self.rl.load_material_default(&self.thread)
    }

    pub fn unload_material(&mut self, material: WeakMaterial) {
        unsafe { self.rl.unload_material(&self.thread, material) }
    }

    pub fn unload_model(&mut self, model: WeakModel) {
        unsafe { self.rl.unload_model(&self.thread, model) }
    }

    pub fn unload_model_animation(&mut self, model_animation: WeakModelAnimation) {
        unsafe {
            self.rl
                .unload_model_animation(&self.thread, model_animation)
        }
    }

    pub fn unload_mesh(&mut self, mesh: WeakMesh) {
        unsafe { self.rl.unload_mesh(&self.thread, mesh) }
    }

    pub fn load_shader(&mut self, vs_filename: Option<&str>, fs_filename: Option<&str>) -> Shader {
        self.rl.load_shader(&self.thread, vs_filename, fs_filename)
    }

    pub fn load_shader_from_memory(
        &mut self,
        vs_code: Option<&str>,
        fs_code: Option<&str>,
    ) -> Shader {
        self.rl
            .load_shader_from_memory(&self.thread, vs_code, fs_code)
    }

    pub fn load_font(&mut self, filename: &str) -> Result<Font, Error> {
        self.rl.load_font(&self.thread, filename)
    }

    pub fn load_font_ex(
        &mut self,
        filename: &str,
        font_size: i32,
        chars: Option<&str>,
    ) -> Result<Font, Error> {
        self.rl
            .load_font_ex(&self.thread, filename, font_size, chars)
    }

    pub fn load_font_from_image(
        &mut self,
        image: &Image,
        key: impl Into<raylib::ffi::Color>,
        first_char: i32,
    ) -> Result<Font, Error> {
        self.rl
            .load_font_from_image(&self.thread, image, key, first_char)
    }

    pub fn load_font_from_memory(
        &mut self,
        file_type: &str,
        file_data: &[u8],
        font_size: i32,
        chars: Option<&str>,
    ) -> Result<Font, Error> {
        self.rl
            .load_font_from_memory(&self.thread, file_type, file_data, font_size, chars)
    }

    pub fn load_texture(&mut self, filename: &str) -> Result<Texture2D, Error> {
        self.rl.load_texture(&self.thread, filename)
    }

    pub fn load_texture_cubemap(
        &mut self,
        image: &Image,
        layout: CubemapLayout,
    ) -> Result<Texture2D, Error> {
        self.rl.load_texture_cubemap(&self.thread, image, layout)
    }

    pub fn load_texture_from_image(&mut self, image: &Image) -> Result<Texture2D, Error> {
        self.rl.load_texture_from_image(&self.thread, image)
    }

    pub fn load_render_texture(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<RenderTexture2D, Error> {
        self.rl.load_render_texture(&self.thread, width, height)
    }

    pub fn unload_texture(&mut self, texture: WeakTexture2D) {
        unsafe { self.rl.unload_texture(&self.thread, texture) }
    }
    pub fn unload_render_texture(&mut self, texture: WeakRenderTexture2D) {
        unsafe { self.rl.unload_render_texture(&self.thread, texture) }
    }
    pub fn load_vr_stereo_config(
        &mut self,
        device: impl Into<raylib::ffi::VrDeviceInfo>,
    ) -> VrStereoConfig {
        self.rl.load_vr_stereo_config(&self.thread, device)
    }

    pub fn set_window_title(&self, title: &str) {
        self.rl.set_window_title(&self.thread, title)
    }
}
