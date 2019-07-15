extern crate render_engine as re;
use re::*;

extern crate nalgebra_glm as glm;
use glm::*;

use crate::WORLD_SIZE;

use crate::rand::Rng;

pub struct Apple {
    pub position: Vec3,
    world_com: WorldCommunicator,
}

impl Apple {
    pub fn from_world_com(world_com: WorldCommunicator) -> Self {
        let mut rng = rand::thread_rng();
        let mut apple = Self {
            position: vec3(0.0, 0.0, 0.0),
            world_com,
        };

        apple.randomize_position();

        apple
    }

    pub fn randomize_position(&mut self) {
        let mut rng = rand::thread_rng();
        let half = WORLD_SIZE as i32 / 2;
        self.position.x = rng.gen_range(-half, half) as f32;
        self.position.y = rng.gen_range(-half, half) as f32;
        self.position.z = rng.gen_range(-half, half) as f32;

        self.update_mesh();
    }

    fn update_mesh(&mut self) {
        self.world_com.delete_object("apple".to_string());

        let verts: Vec<_> = crate::snake::CUBE_VERTICES.iter().map(|vertex| Vertex {
            position: [self.position.x + vertex.position[0], self.position.y + vertex.position[1], self.position.z + vertex.position[2]],
            color: [1.0, 0.0, 0.0],
            normal: vertex.normal,
        })
        .collect();

        self.world_com.add_object_from_verts("apple".to_string(), verts);
    }
}
