use godot::classes::ISprite2D;
use godot::classes::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    /// This is the constructor that must be overrided!
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 450.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // self.self_rotation(delta);
        self.circle_rotation(delta);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}

impl Player {
    // fn self_rotation(&mut self, delta: f64) {
    //     // In GDScript, this would be:
    //     // rotation += angular_speed * delta

    //     let radians = (self.angular_speed * delta) as f32;
    //     self.base_mut().rotate(radians);
    //     // The 'rotate' method requires a f32,
    //     // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    // }

    fn circle_rotation(&mut self, delta: f64) {
        // In GDScript, this would like:
        //
        // rotation += angular_speed * delta
        // var velacity = Vector2.UP.rorated(rotation) * speed
        // position += velocity * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);

        // or verbose:
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }
}
