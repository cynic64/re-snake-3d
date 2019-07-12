use super::*;

#[rustfmt::skip]
const CUBE_CORNER_POSITIONS: [[f32; 3]; 8] = [ [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [-0.5,  0.5, -0.5], [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5], ];

#[rustfmt::skip]
const CUBE_VERTICES: [Vertex; 36] = [ Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, ];

pub struct Snake {
    // the first piece is the head, the last is the tail
    pieces: Vec<Position3D>,
    velocity: Velocity,
    snake_pos_send: Sender<Position3D>,
}

#[derive(Clone)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Snake {
    pub fn with_sending_channel(snake_pos_send: Sender<Position3D>) -> Self {
        Self {
            pieces: vec![
                Position3D { x: 0.0, y: 0.0, z: 0.0 },
                Position3D { x: 1.0, y: 0.0, z: 0.0 },
                Position3D { x: 2.0, y: 0.0, z: 0.0 },
                Position3D { x: 3.0, y: 0.0, z: 0.0 },
                Position3D { x: 4.0, y: 0.0, z: 0.0 },
                Position3D { x: 5.0, y: 0.0, z: 0.0 },
            ],
            velocity: Velocity {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            snake_pos_send,
        }
    }

    pub fn move_pieces(&mut self) {
        // closure borrow checking, grumble grumble
        let velocity = self.velocity.clone();

        self.pieces.iter_mut().for_each(|piece| {
            piece.x += velocity.x;
            piece.y += velocity.y;
            piece.z += velocity.z;
        });

        // send the coordinate of the head to the camera
        self.snake_pos_send.send(self.pieces[0].clone()).unwrap();
    }

    pub fn get_verts(&self) -> Vec<Vertex> {
        self.pieces.iter().enumerate().flat_map(move |(idx, grid_coord)| CUBE_VERTICES.iter().map(move |vertex| Vertex {
            position: [vertex.position[0] + grid_coord.x, vertex.position[1] + grid_coord.y, vertex.position[2] + grid_coord.z],
            color: if idx == 0 {
                    [1.0, 0.0, 0.0]
                } else {
                    let value = ((self.pieces.len() - idx) as f32) / ((self.pieces.len()) as f32);
                    [value, value, value]
                },
            normal: vertex.normal,
        })).collect()
    }
}
