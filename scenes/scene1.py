from raylang import render

scene = {
    'threads'      : 4,
    'image'        : 'scene1.png',
    'width'        : 512,
    'height'       : 512,
    'subsamples'   : 1,
    'background'   : 'black',
    'ambient_light': 'white',

    'default_material' : {
        'k_diffuse': 0.7,
        'k_specular': 0.3,
        'k_ambient': 0.1,
        'n_shininess' : 30.0,
        'color': 'white'
    },

    'camera': {
        'distance'   : 5.0,
        'fov_angle'  : 45.0,
        'location'   : [0.0, 0.0, 5.0],
        'direction'  : [0.0, 0.0, -1.0],
        'up'         : [0.0, 1.0, 0.0]
    },

    'lights': [
    ],

    'objects': {
      'type': 'Group',
      'items': []
    }
}

render(scene)
