use nannou::prelude::*;

mod boid;
use boid::{Boid, BoidType};

fn main() {
    nannou::app(model).update(update).run();
}

const NO_BOIDS: usize = 50;
const NO_PREDATOR: usize = 2;

struct Model {
    boids: Vec<Boid>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1500, 900)
        .view(view)
        .build()
        .unwrap();

    let mut boids = Vec::new();

    for i in 0..NO_BOIDS {
        let boid = Boid::new(
            (random_f32() - 0.5) * 1000.0,
            (random_f32() - 0.5) * 800.0,
            20.0,
            if i < NO_PREDATOR {
                BoidType::Predator
            } else {
                BoidType::Prey
            },
        );
        boids.push(boid);
    }

    Model { boids }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let screen_right = app.window_rect().right() as f32;
    let screen_top = app.window_rect().top() as f32;

    for i in 0..NO_BOIDS {
        let local_boids = model.boids[i].local_boids(&model.boids, i);

        if model.boids[i].nature == BoidType::Prey {
            let alignment = model.boids[i].alignment(&local_boids);
            let cohesion = model.boids[i].cohesion(&local_boids);
            let separation = model.boids[i].separation(&local_boids);

            let predator_avoidance = model.boids[i].avoid_predators(&local_boids);

            // Uncomment the code if you want preys to turn into predators 
            if model.boids[i].convert_to_predator(&local_boids) {
                // model.boids[i].nature = BoidType::Predator
            }

            model.boids[i].acceleration += alignment + cohesion + separation + predator_avoidance;
        } else {
            let catch_prey = model.boids[i].catch_prey(&local_boids);
            let avoid_other_predators = model.boids[i].avoid_predators(&local_boids);

            model.boids[i].acceleration += catch_prey + avoid_other_predators;
        }

        model.boids[i].update();
        model.boids[i].edge(screen_top, screen_right);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);

    for i in 0..NO_BOIDS {
        model.boids[i].show(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
