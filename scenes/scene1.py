from ctypes import *

lib = cdll.LoadLibrary('target/release/libraylangrust.dylib')

class Vec3f(Structure):
  _fields_ = [("x", c_float),
              ("y", c_float),
              ("z", c_float)]

class Scene(Structure):
  _fields_ = []

##

vec3f = lib.vec3f
vec3f.argtypes = [c_float, c_float, c_float]
vec3f.restype = POINTER(Vec3f)

mag = lib.mag
mag.argtypes = [POINTER(Vec3f)]
mag.restype = c_float

read_scene = lib.read_scene
read_scene.argtypes = [c_char_p]
read_scene.restype = POINTER(Scene)

render = lib.render
render.argtypes = [POINTER(Scene)]
render.restype = None

##

v123 = vec3f(1.0, 2.0, 3.0)
print mag(v123)

v345 = Vec3f(3.0, 4.0, 5.0)
print mag(v345)

##

scene = read_scene('scenes/scene0.toml')
render(scene)
