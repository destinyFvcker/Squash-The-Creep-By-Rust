use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    #[export] // use #[export] to export fields to godot editor
    speed: i64,
    #[export]
    jump_impulse: i64,
    #[export]
    bounce_impulse: i64,
    #[export]
    fall_acceleration: i64,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    /// This is the constructor that must be overrided!
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14,
            jump_impulse: 20,
            bounce_impulse: 16,
            fall_acceleration: 75,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {}
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();
}

impl Player {}
