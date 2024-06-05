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

fn update(app: &App, model: &mut Model, _update: Update){
    model.speed += 1.0;
    if model.speed > app.window_rect().right() {
        model.speed = app.window_rect().left();
    }
    
    model.red += 0.01;
    if model.red > 1.0 {
        model.red = 0.0;
    }
    
    model.green += 0.01;
    if model.green > 1.0 {
        model.green = 0.0;
    }
    
    model.blue += 0.01;
    if model.blue > 1.0 {
        model.blue = 0.0;
    }
}

fn view(app: &App,model: &Model, frame: Frame){
    frame.clear(PURPLE);

    let draw = app.draw();
    // draw.background().color(PLUM);
    draw.background().color(rgb(model.red, model.green, model.blue));
    draw.ellipse().color(MAGENTA).w(100.0).h(100.0).x(model.speed);
    draw.to_frame(app, &frame).unwrap();
}
