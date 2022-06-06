// Import
use nannou::prelude::*;
use rand::Rng;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {
    random: rand::rngs::ThreadRng,
    size: u32,
    current: Vec2,
}

// Ran before drawing
fn model(app: &App) -> Model {
    // Set up randomizer
    let mut random = rand::thread_rng();

    // Define startup variables
    let size = 512;
    let current = pt2(
        random.gen_range(0.0..size as f32),
        random.gen_range(0.0..size as f32),
    );

    // Make new window
    app.new_window()
        .size(size, size)
        .view(view)
        .build()
        .unwrap();
    Model {
        random,
        size,
        current,
    }
}

// Update app data struct
fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn setup(draw: &Draw) {}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.main_window();
    let win = window.rect();
    let draw = app.draw().x_y(-win.w() * 0.5, win.h() * 0.5);

    println!("{:?}", model.current);

    // Setup
    if frame.nth() == 0 {
        setup(&draw);
    }

    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).x_y(100.0, 0.0); //.xy(model.current);
    draw.to_frame(app, &frame).unwrap();
}
