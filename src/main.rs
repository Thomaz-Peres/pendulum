fn main() {
    println!("Hello, world!");
}

struct Pendulum {
    // The pendulum angle
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, // The length of the pendulum.
    m: f32, // The mass of the ball.
    g: f32, // The gravity
}

impl Pendulum {
    fn new() {
        
    }

    fn update() {

    }

    fn draw() {

    }
}

mod vector {
    struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        fn new() {
            
        }

        fn add() {
            
        }

        fn set() {
            
        }
    }
}