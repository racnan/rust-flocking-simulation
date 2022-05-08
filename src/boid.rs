use std::ops::{Div, Mul, Sub};

use nannou::prelude::*;

#[derive(Debug, PartialEq)]
pub enum BoidType {
    Predator,
    Prey,
}

#[derive(Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub nature: BoidType,
    pub acceleration: Vec2,
    radius: f32,
    max_velocity: f32,
    min_velocity: f32,
    max_force: f32,
}

impl Boid {
    pub fn new(pos_x: f32, pos_y: f32, radius: f32, boid_type: BoidType) -> Boid {
        Boid {
            position: vec2(pos_x, pos_y),
            velocity: vec2(random_f32() - 0.5, random_f32() - 0.5),
            radius,
            acceleration: vec2(random_f32() - 0.5, random_f32() - 0.5),
            max_velocity: if boid_type == BoidType::Prey {
                5.0
            } else {
                3.0
            },
            min_velocity: if boid_type == BoidType::Prey {
                1.5
            } else {
                1.0
            },
            max_force: if boid_type == BoidType::Prey {
                0.5
            } else {
                1.0
            },
            nature: boid_type,
        }
    }

    pub fn show(&self, draw: &Draw) {
        let red = if self.nature == BoidType::Predator {
            255.0
        } else {
            0.0
        };

        draw.tri()
            .w_h(self.radius, self.radius)
            .x_y(self.position.x, self.position.y)
            .rotate(self.velocity.angle())
            .rgba(red, 0.0, 0.0, 0.85);
    }

    pub fn update(&mut self) {
        self.acceleration = set_max_acc(self.max_force, &self.acceleration);
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity = set_velocity(self.max_velocity, self.min_velocity, &self.velocity);

        self.acceleration = vec2(0.0, 0.0);
    }

    pub fn edge(&mut self, top: f32, right: f32) {
        if self.position.x > right {
            self.position.x = -right;
        } else if self.position.x < -right {
            self.position.x = right
        }
        if self.position.y > top {
            self.position.y = -top;
        } else if self.position.y < -top {
            self.position.y = top
        }
    }

    pub fn local_boids<'a>(&self, all_boids: &'a Vec<Boid>, boid_index: usize) -> Vec<&'a Boid> {
        let perception = if self.nature == BoidType::Predator {
            300.0
        } else {
            100.0
        };

        let mut local_boids = Vec::new();

        for i in 0..all_boids.len() {
            if i != boid_index {
                let distance = self.position.distance(all_boids[i].position);
                if distance <= perception {
                    local_boids.push(&all_boids[i]);
                }
            }
        }

        local_boids
    }

    pub fn alignment(&self, other_boids: &Vec<&Boid>) -> Vec2 {
        let len = other_boids.len();

        if len == 0 {
            return vec2(0.0, 0.0);
        }

        let mut average_velocity = vec2(0.0, 0.0);
        for i in other_boids {
            average_velocity += i.velocity;
        }
        average_velocity = average_velocity.div(len as f32);
        average_velocity.sub(self.velocity) / 2.5
    }

    pub fn cohesion(&self, other_boids: &Vec<&Boid>) -> Vec2 {
        let len = other_boids.len();

        if len == 0 {
            return vec2(0.0, 0.0);
        }

        let mut average_position = vec2(0.0, 0.0);
        for i in other_boids {
            average_position += i.position;
        }
        average_position = average_position.div(len as f32);
        average_position.sub(self.position) / 20.0
    }

    pub fn separation(&self, other_boids: &Vec<&Boid>) -> Vec2 {
        let len = other_boids.len();

        if len == 0 {
            return vec2(0.0, 0.0);
        }

        let mut average_seperation = vec2(0.0, 0.0);
        for i in other_boids {
            let difference_vec = i
                .position
                .sub(self.position)
                .div(self.position.distance(i.position) * 2.0);
            average_seperation -= difference_vec;
        }
        average_seperation * 1.5
    }

    pub fn avoid_predators(&self, other_boids: &Vec<&Boid>) -> Vec2 {
        let len = other_boids.len();

        if len == 0 {
            return vec2(0.0, 0.0);
        }

        let mut avoid_vec = vec2(0.0, 0.0);
        for i in other_boids {
            if i.nature == BoidType::Predator {
                let difference_vec = i
                    .position
                    .sub(self.position)
                    .div(self.position.distance(i.position) * 2.0);
                avoid_vec -= difference_vec;
            }
        }
        avoid_vec * 5.0
    }

    pub fn convert_to_predator(&self, other_boids: &Vec<&Boid>) -> bool {
        let len = other_boids.len();

        if len == 0 {
            return false;
        }
        for i in other_boids {
            if i.nature == BoidType::Predator && self.position.distance(i.position) < 40.0 {
                return true;
            }
        }

        false
    }

    pub fn catch_prey(&self, other_boids: &Vec<&Boid>) -> Vec2 {
        let len = other_boids.len();

        if len == 0 {
            return vec2(0.0, 0.0);
        }

        let mut average_velocity = vec2(0.0, 0.0);
        let mut average_position = vec2(0.0, 0.0);

        let mut no_prey_counter = 0;

        for i in other_boids {
            if i.nature == BoidType::Prey {
                average_velocity += i.velocity;
                average_position += i.position;
            } else {
                no_prey_counter += 1;
            }
        }

        if no_prey_counter == len {
            return vec2(0.0, 0.0);
        }
        
        average_velocity = average_velocity.div(len as f32);
        average_position = average_position.div(len as f32) ;

        average_velocity.sub(self.velocity) + (average_position.sub(self.position))
    }
}

//HELPER FUNCTIONS
fn set_velocity(max_vel: f32, min_vel: f32, vel: &Vec2) -> Vec2 {
    let vel_len = vel.length_squared();

    let mut new_vel = vel.clone();

    if vel_len > max_vel * max_vel {
        new_vel = vel.normalize_or_zero();
        new_vel = new_vel.mul(max_vel);
    } else if vel_len < min_vel * min_vel {
        new_vel = vel.normalize_or_zero();
        new_vel = new_vel.mul(min_vel);
    }
    new_vel
}

fn set_max_acc(max_acc: f32, acc: &Vec2) -> Vec2 {
    let acc_len = acc.length_squared();

    let mut new_acc = acc.clone();

    if acc_len > max_acc * max_acc {
        new_acc = acc.normalize_or_zero();
        new_acc = new_acc.mul(max_acc);
    }
    new_acc
}
