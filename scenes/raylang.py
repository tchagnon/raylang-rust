from ctypes import *
import json

lib = cdll.LoadLibrary('target/release/libraylangrust.dylib')

lib.decode_json_scene.argtypes = [c_char_p]
lib.decode_json_scene.restype = c_void_p

lib.render.argtypes = [c_void_p]
lib.render.restype = None

def render(scene):
  scene_json = json.dumps(scene)
  scene_ptr = lib.decode_json_scene(scene_json)
  lib.render(scene_ptr)

