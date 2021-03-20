use nannou::prelude::*;

struct Model {
    _window: window::Id,
    valence: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    return Model { _window, valence: 3 };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(GREY);
    draw.text("🙂");
    let win = _app.window_rect();
    draw.to_frame(_app, &_frame).unwrap();
}

