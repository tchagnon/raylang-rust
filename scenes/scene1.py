from raylang import *
from math import pi, cos, sin

light_fire = {
    'color'      : 'orange',
    'intensity'  : 2.0,
    'position'   : [0.0, 1.0, 0.0]
}

light0 = {
    'color'    : 'white',
    'intensity': 0.5,
    'position' : [4.0, 4.0, 9.0]
}

mat_default = {
    'k_diffuse'  : 0.7,
    'k_specular' : 0.3,
    'k_ambient'  : 0.1,
    'n_shininess': 30,
    'color'      : 'white'
}

mat_rocks = mat_default.copy()
mat_rocks['color'] = 'gray'

mat_bunny = mat_default.copy()
mat_bunny['color'] = 'cornflowerBlue'

mat_fire     = {
    'k_diffuse': 0.8,
    'k_specular': 0.1,
    'k_ambient': 0.5,
    'n_shininess': 5,
    'color': 'orange'
    }

mat_frog = {
    'k_diffuse': 0.6,
    'k_specular': 1.0,
    'k_ambient': 0.3,
    'n_shininess': 70,
    'color': 'darkGreen'
    }

mat_outhouse     = {
    'k_diffuse': 0.4,
    'k_specular': 0.2,
    'k_ambient': 0.4,
    'n_shininess': 5,
    'color': 'brown'
    }

mat_cow = mat_outhouse.copy()
mat_cow['color'] = 'darkBrown'

scene = {
    'threads'      : 4,
    'image'        : 'scene1.png',
    'width'        : 512,
    'height'       : 512,
    'subsamples'   : 2,
    'background'   : 'black',
    'ambient_light': 'white',

    'camera': {
        'distance'   : 5.0,
        'fov_angle'  : 45.0,
        'location'   : [0.0, 3.0, 5.0],
        'direction'  : [0.0, -0.5, -1.0],
        'up'         : [0.0, 1.0, -0.5]
    },

    'default_material' : mat_default,
    'lights': [light0, light_fire],

    'objects': group([
      material(mat_rocks,
        group([
          translate([cos(t*pi/6.0), 0, sin(t*pi/6.0)],
            sphere(0.08, [0, 0, 0]))
          for t in range(0, 24)])),

      material(mat_default,
        translate([-1.3, 0, 1.3],
          scale([0.15, 0.15, 0.15],
            rotate(45.0, [0.0, 1.0, 0.0],
              mesh('models/teapot.smf', 'smooth'))))),
      material(mat_bunny,
        translate([1.5, 0.5, 1.5],
          rotate(-45.0, [0.0, 1.0, 0.0],
              mesh('models/bound-bunny_1k.smf', 'flat')))),
      material(mat_fire,
        translate([0.3, 0, 0],
          scale([0.007, 0.007, 0.007],
            rotate(-90, [1, 0, 0],
              mesh('models/campfire.smf', 'smooth'))))),
      material(mat_frog,
        translate([1.7, 0.5, -1.7],
          scale([0.5, 0.5, 0.5],
            rotate(45, [0, 1, 0],
              mesh('models/frog.smf', 'smooth'))))),
      material(mat_outhouse,
        translate([3, 0, -8],
          scale([0.02, 0.02, 0.02],
            rotate(-35, [0, 1, 0],
              rotate(-90, [1, 0, 0],
                mesh('models/outhouse.smf', 'smooth')))))),
      material(mat_cow,
        translate([-1.5, 0.6, -1.5],
          scale([2, 2, 2],
            rotate(-45, [0, 1, 0],
              mesh('models/bound-cow.smf', 'smooth'))))),
      ])
}

render(scene)
