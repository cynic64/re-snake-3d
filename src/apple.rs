extern crate render_engine as re;
use re::*;

extern crate nalgebra_glm as glm;
use glm::*;

use crate::WORLD_SIZE;

use crate::rand::Rng;

pub const APPLE_RADIUS: f32 = 5.0;

pub struct Apple {
    pub position: Vec3,
    world_com: WorldCommunicator,
}

impl Apple {
    pub fn from_world_com(world_com: WorldCommunicator) -> Self {
        let mut apple = Self {
            position: vec3(0.0, 0.0, 0.0),
            world_com,
        };

        apple.randomize_position();

        apple
    }

    pub fn randomize_position(&mut self) {
        let mut rng = rand::thread_rng();
        let max = WORLD_SIZE as i32 / 2 - 10;
        self.position.x = rng.gen_range(-max, max) as f32;
        self.position.y = rng.gen_range(-max, max) as f32;
        self.position.z = rng.gen_range(-max, max) as f32;

        self.update_mesh();
    }

    fn update_mesh(&mut self) {
        self.world_com.delete_object("apple".to_string());

        let verts = mesh_gen::create_vertices_for_cube(self.position.into(), APPLE_RADIUS, [1.0, 0.0, 0.0]);
        let spec = ObjectSpec::from_mesh(verts);
        self.world_com.add_object_from_spec("apple".to_string(), spec);
    }
}
