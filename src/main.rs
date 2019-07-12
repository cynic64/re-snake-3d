extern crate render_engine as re;
extern crate nalgebra_glm as glm;

use re::*;
use glm::*;

use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod snake;
use snake::Position3D;

const TIME_BETWEEN_MOVES: f32 = 0.2;

fn main() {
    let (tx, rx): (Sender<Position3D>, Receiver<Position3D>) = mpsc::channel();
    let camera = Box::new(CustomOrbitCamera::with_receiving_channel(rx));
    let mut app = AppBuilder::default()
        .with_multisampling()
        .with_camera(camera)
        .build();

    let mut snake = snake::Snake::with_sending_channel(tx);
    let mut last_move_time = std::time::Instant::now();

    while !app.done {
        app.clear_vertex_buffers();
        let new_verts = snake.get_verts();
        app.new_vbuf_from_verts(&new_verts);

        if get_elapsed(last_move_time) > TIME_BETWEEN_MOVES {
            last_move_time = std::time::Instant::now();
            snake.move_pieces();
        }

        app.draw_frame();
    }

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
    snake_pos_recv: Receiver<Position3D>,
}

impl CustomOrbitCamera {
    fn with_receiving_channel(snake_pos_recv: Receiver<Position3D>) -> Self {
        let center_position = vec3(0.0, 0.0, 0.0);
        let pitch: f32 = 0.0;
        let yaw: f32 = std::f32::consts::PI / 2.0;
        let front = normalize(&vec3(
            pitch.cos() * yaw.cos(),
            pitch.sin(),
            pitch.cos() * yaw.sin(),
        ));
        let right = vec3(0.0, 0.0, 0.0);
        let up = vec3(0.0, 1.0, 0.0);
        let world_up = vec3(0.0, 1.0, 0.0);
        let mouse_sens = 0.0007;
        let orbit_distance = 8.0;

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
            snake_pos_recv,
        }
    }

    fn update(&mut self) {
        self.front = normalize(&vec3(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        ));

        self.right = normalize(&glm::Vec3::cross(&self.front, &self.world_up));
    }

    fn check_channel(&mut self) {
        if let Ok(pos) = self.snake_pos_recv.try_recv() {
            println!("Got new position!: {:?}", pos);
            self.center_position = vec3(pos.x, pos.y, pos.z);
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

    fn handle_input(&mut self, events: &[Event], _keys_down: &KeysDown, _delta: f32) {
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

                self.update();
            }
        });

        self.check_channel();
    }
}
