//! C library interface for creating Scenes and rendering them
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate image;
extern crate libc;
extern crate time;

mod color;
mod math;
mod mesh;
mod primitive;
mod ray_tracer;
mod bounding_box;
mod scene;

use libc::c_char;
use std::ffi::CStr;
use scene::Scene;

#[no_mangle]
pub extern "C" fn decode_json_scene(json: *const c_char) -> *const Scene {
    unsafe {
        let json_str = CStr::from_ptr(json).to_str()
            .expect("Error converting json to str");
        Box::into_raw(
            Box::new(
                Scene::decode_json(json_str).prepare()))
    }

}

#[no_mangle]
pub extern "C" fn render(scene: *const Scene) {
    unsafe {
        (*scene).render();
        println!("Wrote file {:?}", (*scene).image);
    }
}
