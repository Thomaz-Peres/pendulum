use speedy2d::window::WindowHandler;
use vector::Vector;

fn main() {
    println!("Hello, world!");
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

    fn draw() {}
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

struct MyWindowHandler {}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
    }
}
