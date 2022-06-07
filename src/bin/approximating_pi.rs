// Import
use nannou::prelude::*;
use rand::thread_rng;
use rand::Rng;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {
    pi: f64,
    total: f64,
    circle: f64,
    record_pi: f64,
    record_diff: f64,
    new_points: Vec<Point>,
}

const SIZE: u32 = 512;
const RADIUS: f32 = SIZE as f32 / 2.0;
const BUFFER_FRAMES: i32 = 2048;

// Ran before drawing
fn model(app: &App) -> Model {
    // Some variables to start with
    let pi = 0.0;
    let total = 0.0;
    let circle = 0.0;
    let record_pi = 0.0;
    let record_diff = 0.0;
    let new_points: Vec<Point> = Vec::new();

    // Make new window
    app.new_window()
        .size(SIZE, SIZE)
        .view(view)
        .build()
        .unwrap();

    // Retunr application start state
    Model {
        pi,
        total,
        circle,
        record_pi,
        new_points,
        record_diff,
    }
}

struct Point {
    cords: Vec2,
    color: nannou::color::Rgb<u8>,
}

// Update app data struct
fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut random = thread_rng();
    let size = SIZE as f32;
    model.new_points.clear();
    for _ in 0..BUFFER_FRAMES {
        model.total += 1.0;
        let point = vec2(
            random.gen_range(-size..size) * 0.5,
            random.gen_range(-size..size) * 0.5,
        );

        let distance = point.distance(pt2(0.0, 0.0));

        match distance < RADIUS {
            true => {
                model.circle += 1.0;
                model.new_points.push(Point {
                    cords: point,
                    color: GREEN,
                })
            }
            false => model.new_points.push(Point {
                cords: point,
                color: RED,
            }),
        }

        model.pi = 4.0 * (model.circle / model.total);
        model.record_diff = (PI_F64 - model.record_pi).abs();
        let difference = (PI_F64 - model.pi).abs();

        if difference < model.record_diff {
            model.record_diff = difference;
            model.record_pi = model.pi;
            println!(
                "New record: {}\t diff with real {}",
                model.record_pi, difference
            );
        }
    }
}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    // Setup window draw surface
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }

    for point in &model.new_points {
        draw.ellipse()
            .radius(0.1)
            .xy(point.cords)
            .color(point.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
