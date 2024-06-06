use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1024, 720).run();
}

fn view(app: &App, frame: Frame){
    let draw = app.draw();
    let coeff : u64 = 2;
    draw.background().color(BLACK);
    draw.rect().w_h(200.0, (frame.nth() * coeff) as f32).color(RED);
    draw.to_frame(app, &frame).unwrap();
}