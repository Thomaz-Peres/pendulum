use speedy2d::{color::Color, window::WindowHandler, Graphics2D, Window};
use vector::Vector;

fn main() {
    let window: Window = Window::new_centered("Pendulum", (800, 400)).unwrap();

    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.00),
    };

    window.run_loop(win);
}

struct Pendulum {
    // Position of the pendulum
    origin: Vector,

    // Position of the ball
    position: Vector,

    // The pendulum angle
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, // The length of the pendulum.
    m: f32, // The mass of the ball.
    g: f32, // The gravity
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: (0.0),
            angular_acceleration: (0.0),
            r: (r),
            m: 1.0,
            g: 1.5,
        }
    }

    fn update(&mut self) {
        // Pendulum equation to calculate the angular acceleration
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        // The angular velocity is the angular velocity plus the angular acceleration
        self.angular_velocity += self.angular_acceleration;

        // The angle is the angle plus the angular velocity
        self.angle += self.angular_velocity;

        // The position is the polar coordinates translated to cartesian coordinates.
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        // The final position of the ball in the canvas is the origin of the pendulum plus the position vector
        self.position.add(&self.origin);
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        // Clear the screen every frame.
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        self.p.draw(graphics);

        helper.request_redraw();
    }
}
