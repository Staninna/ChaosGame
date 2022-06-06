// Import
use nannou::prelude::*;
use rand::Rng;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {}

// Ran before drawing
fn model(app: &App) -> Model {
    // Make new window
    app.new_window().size(512, 512).view(view).build().unwrap();
    Model {}
}

// Update app data struct
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.main_window();
    let win = window.rect();
    let draw = app.draw().x_y(-win.w() * 0.5, -win.h() * 0.5);

    draw.background().color(PLUM);
    draw.to_frame(app, &frame).unwrap();
}
