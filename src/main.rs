extern crate nalgebra_glm as glm;
extern crate render_engine as re;

use glm::*;
use re::*;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod snake;
use snake::Position3D;
use snake::CUBE_VERTICES;

const TIME_BETWEEN_MOVES: f32 = 0.1;
const WORLD_SIZE: f32 = 64.0;

fn main() {
    let (snake_pos_send, snake_pos_recv): (Sender<Position3D>, Receiver<Position3D>) =
        mpsc::channel();
    let (camera_angle_send, camera_angle_recv): (Sender<Vec3>, Receiver<Vec3>) = mpsc::channel();
    let camera = Box::new(CustomOrbitCamera::with_channels(
        snake_pos_recv,
        camera_angle_send,
    ));
    let mut app = AppBuilder::default()
        .with_multisampling()
        .with_camera(camera)
        .build();

    let mut world = World::from_creator(app.create_new_vbuf_creator());
    let mut world_com = world.get_communicator();

    let bounding_cube_verts: Vec<_> = CUBE_VERTICES
        .iter()
        .map(|vertex| Vertex {
            position: [
                vertex.position[0] * WORLD_SIZE,
                vertex.position[1] * WORLD_SIZE,
                vertex.position[2] * WORLD_SIZE,
            ],
            color: [1.0, 0.8, 0.8],
            normal: vertex.normal,
        })
        .collect();
    world_com.add_object_from_verts("bounding box".to_string(), bounding_cube_verts);

    let mut snake =
        snake::Snake::with_channels_and_com(snake_pos_send, camera_angle_recv, world_com);
    let mut last_move_time = std::time::Instant::now();

    while !app.done && !snake.is_dead {
        world.check_for_commands();
        app.set_vertex_buffers(world.get_vbufs());

        if get_elapsed(last_move_time) > TIME_BETWEEN_MOVES {
            last_move_time = std::time::Instant::now();
            snake.move_pieces();
        }

        app.draw_frame();
    }

    println!("Noob, you died lul");
    app.print_fps();
}

struct CustomOrbitCamera {
    center_position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,
    // pitch and yaw are in radians
    pitch: f32,
    yaw: f32,
    mouse_sens: f32,
    orbit_distance: f32,
    normal_orbit_distance: f32,
    snake_pos_recv: Receiver<Position3D>,
    camera_angle_send: Sender<Vec3>,
    target_pos: Vec3,
    max_dist: f32,
    movement_speed: f32,
}

impl CustomOrbitCamera {
    fn with_channels(
        snake_pos_recv: Receiver<Position3D>,
        camera_angle_send: Sender<Vec3>,
    ) -> Self {
        let center_position = vec3(0.0, 0.0, 0.0);
        let pitch: f32 = 0.0;
        let yaw: f32 = 0.0;
        let front = normalize(&vec3(
            pitch.cos() * yaw.cos(),
            pitch.sin(),
            pitch.cos() * yaw.sin(),
        ));
        let right = vec3(0.0, 0.0, 0.0);
        let up = vec3(0.0, 1.0, 0.0);
        let world_up = vec3(0.0, 1.0, 0.0);
        let mouse_sens = 0.0007;
        let orbit_distance = 32.0;
        let normal_orbit_distance = 32.0;
        let target_pos = center_position.clone();
        let max_dist = 2.0;
        let movement_speed = 0.5;

        CustomOrbitCamera {
            center_position,
            front,
            up,
            right,
            world_up,
            pitch,
            yaw,
            mouse_sens,
            orbit_distance,
            normal_orbit_distance,
            snake_pos_recv,
            camera_angle_send,
            target_pos,
            max_dist,
            movement_speed,
        }
    }

    fn update(&mut self) {
        self.front = normalize(&vec3(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        ));

        self.right = normalize(&glm::Vec3::cross(&self.front, &self.world_up));

        self.camera_angle_send.send(self.front.clone()).unwrap();
    }

    fn check_channel(&mut self) {
        if let Ok(pos) = self.snake_pos_recv.try_recv() {
            self.target_pos = vec3(pos.x, pos.y - 10.0, pos.z);
        }
    }
}

impl Camera for CustomOrbitCamera {
    fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        let farther_front = self.front * self.orbit_distance;
        look_at(
            &(self.center_position + farther_front),
            &self.center_position,
            &self.up,
        )
        .into()
    }

    fn handle_input(&mut self, events: &[Event], _keys_down: &KeysDown, delta: f32) {
        events.iter().for_each(|ev| {
            if let Some(mouse_movement) = winit_event_to_mouse_movement(ev) {
                let (x, y) = mouse_movement;

                self.pitch += y * self.mouse_sens;
                self.yaw += x * self.mouse_sens;
                let halfpi = std::f32::consts::PI / 2.0;
                let margin = 0.01;
                let max_pitch = halfpi - margin;

                if self.pitch > max_pitch {
                    self.pitch = max_pitch;
                } else if self.pitch < -max_pitch {
                    self.pitch = -max_pitch;
                }
            }
        });

        self.check_channel();

        let move_dir = vec3(
            self.target_pos.x - self.center_position.x,
            self.target_pos.y - self.center_position.y,
            self.target_pos.z - self.center_position.z,
        );
        let movement_vec = vec3(
            move_dir.x * delta * self.movement_speed,
            move_dir.y * delta * self.movement_speed,
            move_dir.z * delta * self.movement_speed,
        );
        self.center_position.x += movement_vec.x;
        self.center_position.y += movement_vec.y;
        self.center_position.z += movement_vec.z;

        let max = WORLD_SIZE / 2.0;
        // figure out where min and abs are for this stuff
        let dist_x = if self.center_position.x > 0.0 {
            max - self.center_position.x
        } else {
            max + self.center_position.x
        };

        let dist_y = if self.center_position.y > 0.0 {
            max - self.center_position.y
        } else {
            max + self.center_position.y
        };

        let dist_z = if self.center_position.z > 0.0 {
            max - self.center_position.z
        } else {
            max + self.center_position.z
        };

        let min_dist = if dist_x < dist_y && dist_x < dist_z {
            dist_x
        } else if dist_y < dist_x && dist_y < dist_z {
            dist_y
        } else {
            dist_z
        };

        if min_dist > self.normal_orbit_distance {
            self.orbit_distance = self.normal_orbit_distance;
        } else {
            self.orbit_distance = min_dist;
        }

        let distance = distance(&self.center_position, &self.target_pos);
        if distance > self.max_dist {
            self.movement_speed += 0.01;
        } else {
            self.movement_speed -= 0.01;
        }

        if self.movement_speed > 1.0 {
            self.movement_speed = 1.0;
        }

        self.update();
    }
}
