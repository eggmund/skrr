use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, DrawParam, DrawMode, Rect};
use na::{Vector2, Point2, Rotation2};

use std::f32::consts::PI;

const TWO_PI: f32 = PI * 2.0;
const CAR_DIMS: (f32, f32) = (20.0, 30.0);  // (width, height)
const MAX_TURN_RATE: f32 = PI/2.0;
const MAX_WHEEL_ANGULAR_ACC: f32 = PI * 1000.0;

const CAR_MASS: f32 = 1000.0;
const CAR_NORMAL_FORCE: f32 = CAR_MASS * 9.81;

const STATIC_FRIC_COEF: f32 = 1.0;
const KIN_FRIC_COEF: f32 = 0.6;     // Friction when sliding

const WHEEL_DIST_FROM_CENTRE: f32 = CAR_DIMS.1/3.0;

// Treat the car as as single wheel
pub struct Car {
    pos: Point2<f32>,
    vel: Vector2<f32>,
    body_angle: f32,    // Angle from +x axis. -ve = cw
    heading: Vector2<f32>,
    throttle: f32,
    steering: f32,
    body_angular_vel: f32,
    // --- Wheel ---
    angular_vel: f32,
    angular_acc: f32,
}

impl Car {
    pub fn new(pos: Point2<f32>) -> Self {
        Self {
            pos,
            vel: Vector2::zeros(),
            body_angle: 0.0,
            heading: Vector2::x(),
            throttle: 0.0,
            steering: 0.0,
            body_angular_vel: 0.0,

            angular_vel: 0.0,
            angular_acc: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Get current heading from angle. unit vector
        self.heading = Vector2::new(self.body_angle.cos(), self.body_angle.sin());

        // Check for steering/throttle.
        if self.steering != 0.0 {
            self.steer(dt);
        }

        if self.throttle != 0.0 {
            self.accelerate(dt);
        }

        self.calculate_velocity(dt);
        self.pos += self.vel * dt;
        self.body_angle += self.body_angular_vel * dt;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(0.0, 0.0, CAR_DIMS.1, CAR_DIMS.0),
            [0.4, 0.4, 0.8, 1.0].into(),
        )?;

        graphics::draw(
            ctx,
            &rect,
            DrawParam::default()
                .rotation(self.body_angle)
                .offset([0.5 * CAR_DIMS.1, 0.5 * CAR_DIMS.0])
                .dest([self.pos.x, self.pos.y])

        )?;

        Ok(())
    }

    fn calculate_velocity(&mut self, dt: f32) {
        use crate::tools;
        // Find if it is sliding
        // v_w = v + wrh
        // Wheel radius is 1
        let angular_vel_vec = self.angular_vel * self.heading;
        let point_of_contact_vel = self.vel + angular_vel_vec;

        // If there is no friction, there is no acceleration
        if let Some(contact_vel_norm) = point_of_contact_vel.try_normalize(0.0) {
            println!("Point of contact vel: {:?}", point_of_contact_vel);
    
            // Fr = -v_norm * normal force * friction coefficient.
            let friction: Vector2<f32> = contact_vel_norm * CAR_NORMAL_FORCE * STATIC_FRIC_COEF;
            println!("Friction: {:?}, norm: {:?}", friction, contact_vel_norm);

            // // Apply the frictional force as a moment WHEEL_DIST_FROM_CENTRE away from the centre
            // // the direction will be the determinant of -h with force
            // // T = mass * a, where a = angular acceleration. T/m = a, a * dt = dw.
            // // vecT = -WHEEL_DIST(h x F)
            // let torque: f32 = -WHEEL_DIST_FROM_CENTRE * tools::vec_determinant(&self.heading, &friction);
            // self.body_angular_vel -= (torque/CAR_MASS) * dt;

            // T = r x F = r.x * f.y - r.y * f.x
            // r = -h * WHEEL_DIST
            let torque_dist_vec = -self.heading * WHEEL_DIST_FROM_CENTRE;
            let torque = (torque_dist_vec.x - );
            // When 1.0 - torque.normalize() * dist
    
            // F = ma, a = F/m
            self.vel += (-friction/CAR_MASS) * dt;
        }
    }

    fn accelerate(&mut self, dt: f32) {
        self.angular_acc = MAX_WHEEL_ANGULAR_ACC * self.throttle;
        self.angular_vel += self.angular_acc * dt;
    }

    fn steer(&mut self, dt: f32) {
        self.body_angular_vel += self.steering * MAX_TURN_RATE * dt;
    }

    pub fn set_throttle(&mut self, throttle: f32) {
        self.throttle = throttle;
    }

    pub fn reset_throttle(&mut self) {
        self.set_throttle(0.0)
    }

    pub fn set_steering(&mut self, steering: f32) {
        self.steering = steering;
    }

    pub fn reset_steering(&mut self) {
        self.set_steering(0.0)
    }
}
