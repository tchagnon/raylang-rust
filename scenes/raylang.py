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

def translate(vector, child):
  return {
    'type': 'Transform',
    'transform': {
      'type': 'Translate',
      'vector': vector
    },
    'child': child
  }

def scale(vector, child):
  return {
    'type': 'Transform',
    'transform': {
      'type': 'Scale',
      'vector': vector
    },
    'child': child
  }

def rotate(angle, axis, child):
  return {
    'type': 'Transform',
    'transform': {
      'type': 'Rotate',
      'degrees': angle,
      'axis': axis
    },
    'child': child
  }

def group(items):
  return {
    'type': 'Group',
    'items': items
  }

def material(m, child):
  return {
    'type': 'Material',
    'material': m,
    'child': child
  }

def mesh(smf_file, shading):
  return {
    'type': 'Mesh',
    'mesh': smf_file,
    'shading': shading
  }

def sphere(radius, center):
  return {
    'type': 'Primitive',
    'primitive': 'Sphere',
    'radius': radius,
    'center': center
  }
