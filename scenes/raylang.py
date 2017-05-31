from ctypes import *
import json
import platform

extension = {
  'Darwin': '.dylib',
  'Linux': '.so',
  'Windows': '.dll'
}[platform.system()]

lib = cdll.LoadLibrary('target/release/libraylangrust' + extension)

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
    'Transform': {
      'transform': {
        'Translate': vector
      },
      'child': child
    }
  }

def scale(vector, child):
  return {
    'Transform': {
      'transform': {
        'Scale': vector
      },
      'child': child
    }
  }

def rotate(angle, axis, child):
  return {
    'Transform': {
      'transform': {
        'Rotate': {
          'angle': angle,
          'axis': axis
        }
      },
      'child': child
    }
  }

def group(items):
  return {
    'Group': items
  }

def material(m, child):
  return {
    'Material': {
      'material': m,
      'child': child
    }
  }

def mesh(smf_file, shading):
  return {
    'LoadMesh': {
      'file': smf_file,
      'shading': shading
    }
  }

def sphere(radius, center):
  return {
    'Primitive': {
      'Sphere': {
        'radius': radius,
        'center': center
      }
    }
  }
