use super::*;

#[rustfmt::skip]
const CUBE_CORNER_POSITIONS: [[f32; 3]; 8] = [ [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [-0.5,  0.5, -0.5], [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5], ];

#[rustfmt::skip]
pub const CUBE_VERTICES: [Vertex; 36] = [ Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[3], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[7], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[2], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[6], color: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 1.0] }, Vertex { position: CUBE_CORNER_POSITIONS[4], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[0], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, Vertex { position: CUBE_CORNER_POSITIONS[1], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] }, ];

pub struct Snake {
    // the first piece is the head, the last is the tail
    pieces: Vec<Vec3>,
    velocity: Vec3,
    snake_pos_send: Sender<Vec3>,
    camera_angle_recv: Receiver<Vec3>,
    pub is_dead: bool,
    counter: u32,
    world_com: WorldCommunicator,
    pieces_to_grow: u32,
}

const SNAKE_PIECE_SIZE: f32 = 3.0;

impl Snake {
    pub fn with_channels_and_com(
        snake_pos_send: Sender<Vec3>,
        camera_angle_recv: Receiver<Vec3>,
        world_com: WorldCommunicator,
    ) -> Self {
        Self {
            pieces: vec![
                vec3(0.0, 0.0, 0.0),
                vec3(1.0, 0.0, 0.0),
                vec3(2.0, 0.0, 0.0),
                vec3(3.0, 0.0, 0.0),
                vec3(4.0, 0.0, 0.0),
                vec3(5.0, 0.0, 0.0),
            ],
            velocity: vec3(-1.0, 0.0, 0.0),
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

        let pos_of_tail = self.pieces.last().unwrap().clone();

        // move all except head by shifting each piece to the position of the piece in front of it
        for idx in (1..self.pieces.len()).rev() {
            self.pieces[idx] = self.pieces[idx - 1].clone();
        }
        // move head
        self.pieces[0] += self.velocity;

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
        if self.pieces.len() < 8 {
            return;
        }

        let mut p_iter = self.pieces.iter();
        // skip the first 7 pieces
        p_iter.next();
        p_iter.next();
        p_iter.next();
        p_iter.next();
        p_iter.next();
        p_iter.next();
        p_iter.next();

        if p_iter.any(|piece| {
            distance(
                &vec3(piece.x, piece.y, piece.z),
                &self.pieces[0]
            ) < SNAKE_PIECE_SIZE
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
            .flat_map(move |(idx, piece_pos)| {
                CUBE_VERTICES.iter().map(move |vertex| Vertex {
                    position: [
                        vertex.position[0] * SNAKE_PIECE_SIZE + piece_pos.x,
                        vertex.position[1] * SNAKE_PIECE_SIZE + piece_pos.y,
                        vertex.position[2] * SNAKE_PIECE_SIZE + piece_pos.z,
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
            self.velocity = vec3(-latest_angle.x, -latest_angle.y, -latest_angle.z);
        }
    }

    pub fn check_if_ate_apple(&self, apple: &apple::Apple) -> bool {
        self.pieces.iter().any(|piece| {
            distance(
                piece,
                &apple.position,
            ) < SNAKE_PIECE_SIZE
        })
    }

    pub fn grow(&mut self) {
        self.pieces_to_grow = 20;
    }
}
