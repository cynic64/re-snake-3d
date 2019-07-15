use super::*;

#[rustfmt::skip]
const CUBE_CORNER_POSITIONS: [[f32; 3]; 8] = [ [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [-0.5,  0.5, -0.5], [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5], ];

#[rustfmt::skip]
pub const CUBE_VERTICES: [Vertex; 36] = [ Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, ];

pub struct Snake {
    // the first piece is the head, the last is the tail
    pieces: Vec<Position3D>,
    velocity: Velocity,
    snake_pos_send: Sender<Position3D>,
    camera_angle_recv: Receiver<Vec3>,
    pub is_dead: bool,
    counter: u32,
    world_com: WorldCommunicator,
    pieces_to_grow: u32,
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
    pub fn with_channels_and_com(
        snake_pos_send: Sender<Position3D>,
        camera_angle_recv: Receiver<Vec3>,
        world_com: WorldCommunicator,
    ) -> Self {
        Self {
            pieces: vec![
                Position3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Position3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Position3D {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                Position3D {
                    x: 3.0,
                    y: 0.0,
                    z: 0.0,
                },
                Position3D {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Position3D {
                    x: 5.0,
                    y: 0.0,
                    z: 0.0,
                },
            ],
            velocity: Velocity {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            snake_pos_send,
            camera_angle_recv,
            is_dead: false,
            counter: 0,
            world_com,
            pieces_to_grow: 0,
        }
    }

    pub fn move_pieces(&mut self) {
        // check if we've started going in a different direction because the user changed the camera angle
        self.update_velocity();

        // closure borrow checking, grumble grumble
        let velocity = self.velocity.clone();

        let pos_of_tail = self.pieces.last().unwrap().clone();

        // move all except head by shifting each piece to the position of the piece in front of it
        for idx in (1..self.pieces.len()).rev() {
            self.pieces[idx] = self.pieces[idx - 1].clone();
        }
        // move head
        self.pieces[0].x += velocity.x;
        self.pieces[0].y += velocity.y;
        self.pieces[0].z += velocity.z;

        // duplicate tail to grow if necessary
        if self.counter % 3 == 0 {
            self.pieces.push(pos_of_tail);
        } else if self.pieces_to_grow > 0 {
            self.pieces.push(pos_of_tail);
            self.pieces_to_grow -= 1;
        }

        // send the coordinate of the head to the camera
        self.snake_pos_send.send(self.pieces[0].clone()).unwrap();

        self.check_if_dead();
        self.update_mesh();

        // increment counter
        self.counter += 1;
    }

    fn update_mesh(&mut self) {
        let verts = self.get_verts();
        self.world_com.delete_object("snake".to_string());
        self.world_com
            .add_object_from_verts("snake".to_string(), verts);
    }

    fn check_if_dead(&mut self) {
        // compare all pieces except the head to the head's position to see if it is too close to any
        if self.pieces.len() < 3 {
            return;
        }

        let mut p_iter = self.pieces.iter();
        // skip the first 2 pieces
        p_iter.next();
        p_iter.next();

        if p_iter.any(|piece| {
            distance(
                &vec3(piece.x, piece.y, piece.z),
                &vec3(self.pieces[0].x, self.pieces[0].y, self.pieces[0].z),
            ) < 1.0
        }) {
            self.is_dead = true;
        }

        // make sure the head is in bounds
        let max = WORLD_SIZE / 2.0;
        if self.pieces[0].x > max
            || self.pieces[0].x < -max
            || self.pieces[0].y > max
            || self.pieces[0].y < -max
            || self.pieces[0].z > max
            || self.pieces[0].z < -max
        {
            self.is_dead = true;
        }
    }

    fn get_verts(&self) -> Vec<Vertex> {
        let snake_verts_iter = self
            .pieces
            .iter()
            .enumerate()
            .flat_map(move |(idx, grid_coord)| {
                CUBE_VERTICES.iter().map(move |vertex| Vertex {
                    position: [
                        vertex.position[0] + grid_coord.x,
                        vertex.position[1] + grid_coord.y,
                        vertex.position[2] + grid_coord.z,
                    ],
                    color: if idx == 0 {
                        [0.5, 0.8, 1.0]
                    } else {
                        let value =
                            ((self.pieces.len() - idx) as f32) / ((self.pieces.len()) as f32);
                        [value, value, value]
                    },
                    normal: vertex.normal,
                })
            });

        snake_verts_iter.collect()
    }

    pub fn update_velocity(&mut self) {
        if let Some(latest_angle) = self.camera_angle_recv.try_iter().last() {
            self.velocity = Velocity {
                x: -latest_angle.x,
                y: -latest_angle.y,
                z: -latest_angle.z,
            };
        }
    }

    pub fn check_if_ate_apple(&self, apple: &apple::Apple) -> bool {
        self.pieces.iter().any(|piece| {
            distance(
                &vec3(piece.x, piece.y, piece.z),
                &apple.position,
            ) < 1.0
        })
    }

    pub fn grow(&mut self) {
        self.pieces_to_grow = 20;
    }
}
