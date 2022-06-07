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
    new_points: Vec<Vec2>,
}

const BUFFER_FRAMES: i32 = 2048;
const SIZE: i32 = 512;
const ORIGINS: i32 = 3;

// Ran before drawing
fn model(app: &App) -> Model {
    // Dynamically generated variables
    let mut random = rand::thread_rng();
    let current = pt2(
        random.gen_range(0.0..SIZE as f32),
        random.gen_range(0.0..SIZE as f32),
    );
    let radius = SIZE as f32 / 2.0;
    let points = (0..=360)
        .step_by(360 / ORIGINS as usize)
        .map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = (radian.sin() * radius) + (SIZE / 2) as f32;
            let y = (radian.cos() * radius) + (SIZE / 2) as f32;
            pt2(x, y)
        })
        .collect::<Vec<_>>();

    // Make new window
    app.new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .unwrap();
    Model {
        current,
        points,
        new_points: Vec::new(),
    }
}

// Update app data struct
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.new_points.clear();
    for _ in 0..BUFFER_FRAMES {
        model.current = model.current.lerp(
            model
                .points
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone(),
            0.5,
        );
        model.new_points.push(model.current);
    }
}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    // Setup window draw surface
    let window = app.main_window();
    let win = window.rect();
    let draw = app.draw().x_y(-win.w() * 0.5, -win.h() * 0.5);

    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }

    for point in model.new_points.iter() {
        draw.ellipse().radius(0.1).xy(*point).color(BLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}

// TODO when https://github.com/nannou-org/nannou/issues/383 is merged add gui and other modes
