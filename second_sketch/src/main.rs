use nannou::{draw::Context, prelude::*};

fn main() {
    nannou::sketch(view).run()
        
}

fn view(app: &App, frame: Frame) {
    
    let draw = app.draw();
    draw.background().color(BLACK);
    
    let win = app.window_rect();
    let t = app.time;
    
    let n_points = 2;
    let weight = 1.0;
    let hz = 1.0;
    
    let mut vertices = Vec::new();
    for i in 0..n_points {
        let x = map_range(i, 0, n_points - 1, win.left(), win.right());
        let fract = i as f32 / n_points as f32;
        let amp = (t + fract * hz * TAU).sin();
        let y = map_range(amp, -1.0, 1.0, win.bottom() * 0.75, win.top() * 0.75);
        let point = pt2(x, y);
        let color = srgba(1.0, 0.0, 0.0, 1.0);
        vertices.push((point, color));
    }
    
    draw.polyline()
        .weight(weight)
        .join_round()
        .points_colored(vertices);
    
    draw.to_frame(app, &frame).unwrap();
    
}