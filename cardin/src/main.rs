use nannou::prelude::*;

struct Model {
    particles: Vec<Particle>,
    mouse_pos: Point2,
}

struct Particle {
    position: Point2,
    velocity: Vec2,
    color: Rgb,
}

impl Particle {
    fn new(pos: Point2) -> Self {
        let random_velocity = vec2(
            random_range(-2.0, 2.0),
            random_range(-2.0, 2.0),
        );
        
        Particle {
            position: pos,
            velocity: random_velocity,
            color: rgb(
                random_range(0.5, 1.0),
                random_range(0.5, 1.0),
                random_range(0.5, 1.0),
            ),
        }
    }

    fn update(&mut self, mouse_pos: Point2) {
        let direction = mouse_pos - self.position;
        let distance = direction.length();
        if distance > 0.0 {
            let force = direction.normalize() * (50.0 / distance.max(50.0));
            self.velocity += force * 0.5;
        }

        self.velocity *= 0.99;
        self.position += self.velocity;
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    let particles = (0..50)
        .map(|_| {
            Particle::new(pt2(
                random_range(-400.0, 400.0),
                random_range(-300.0, 300.0),
            ))
        })
        .collect();

    Model {
        particles,
        mouse_pos: pt2(0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.mouse_pos = app.mouse.position();

    for particle in &mut model.particles {
        particle.update(model.mouse_pos);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);

    for particle in &model.particles {
        draw.ellipse()
            .xy(particle.position)
            .w_h(10.0, 10.0)
            .color(particle.color);
    }

    draw.ellipse()
        .xy(model.mouse_pos)
        .w_h(20.0, 20.0)
        .color(BLUE)
        .stroke(WHITE)
        .stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();
}
