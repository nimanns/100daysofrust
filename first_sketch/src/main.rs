use nannou::{draw::background, prelude::*};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    speed: f32,
    red: f32,
    green: f32,
    blue: f32,
}

fn model(_app: &App)->Model {
    Model {
        speed: 0.0,
        red: 0.0,
        green: 0.5,
        blue: 0.2,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update){
    _model.speed += 1.0;
    if _model.speed > _app.window_rect().right() {
        _model.speed = _app.window_rect().left();
    }
    
    _model.red += 0.01;
    if _model.red > 1.0 {
        _model.red = 0.0;
    }
    
    _model.green += 0.01;
    if _model.green > 1.0 {
        _model.green = 0.0;
    }
    
    _model.blue += 0.01;
    if _model.blue > 1.0 {
        _model.blue = 0.0;
    }
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);

    let draw = _app.draw();
    // draw.background().color(PLUM);
    draw.background().color(rgb(_model.red, _model.green, _model.blue));
    draw.ellipse().color(MAGENTA).w(100.0).h(100.0).x(_model.speed);
    draw.to_frame(_app, &frame).unwrap();
}
