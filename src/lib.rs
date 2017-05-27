//! C library interface for creating Scenes and rendering them
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate image;
extern crate toml;
extern crate libc;

mod color;
mod math;
mod mesh;
mod primitive;
mod ray_tracer;
mod scene;

use libc::c_char;
use std::ffi::CStr;
use scene::Scene;

#[no_mangle]
pub extern fn decode_json_scene(json: *const c_char) -> *const Scene {
    unsafe {
        let json_str = CStr::from_ptr(json).to_str()
            .expect("Error converting json to str");
        Box::into_raw(
            Box::new(
                Scene::decode_json(json_str).prepare()))
    }

}

#[no_mangle]
pub extern fn render(scene: *const Scene) {
    unsafe {
        (*scene).render();
        println!("Wrote file {:?}", (*scene).image);
    }
}
