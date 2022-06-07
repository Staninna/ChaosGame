// Import
use nannou::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {
    current: Vec2,
    points: Vec<Vec2>,
}

// Ran before drawing
fn model(app: &App) -> Model {
    // Configurable parameters
    let size = 512;
    let origins = 3;

    // Dynamically generated variables
    let mut random = rand::thread_rng();
    let current = pt2(
        random.gen_range(0.0..size as f32),
        random.gen_range(0.0..size as f32),
    );
    let radius = size as f32 / 2.0;
    let points = (0..=360)
        .step_by(360 / origins as usize)
        .map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = (radian.sin() * radius) + (size / 2) as f32;
            let y = (radian.cos() * radius) + (size / 2) as f32;
            pt2(x, y)
        })
        .collect::<Vec<_>>();

    // Make new window
    app.new_window()
        .size(size, size)
        .view(view)
        .build()
        .unwrap();
    Model { current, points }
}

// Update app data struct
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.current = model.current.lerp(
        model
            .points
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone(),
        0.5,
    );
}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    // Setup window draw surface
    let window = app.main_window();
    let win = window.rect();
    let draw = app.draw().x_y(-win.w() * 0.5, -win.h() * 0.5);

    if frame.nth() == 0 {
        // Draw first frame
        draw.background().color(PLUM);
    }

    draw.ellipse().radius(0.5).xy(model.current).color(BLUE);
    draw.to_frame(app, &frame).unwrap();
}
