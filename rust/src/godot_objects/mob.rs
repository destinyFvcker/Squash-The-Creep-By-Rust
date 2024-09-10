//! Enemy entity in 《squash the creep》
//! implement CharacterBody3D

use std::f64::consts::PI;

use crate::config;
use godot::classes::AnimationPlayer;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::global::randf_range;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Mob {
    #[export]
    speed: f32,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self { speed: 0f32, base }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Mob {
    #[func]
    fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut()
            .look_at_from_position(start_position, player_position);
        self.base_mut()
            .rotate_y(randf_range(-PI / 4f64, PI / 4f64) as f32);

        let random_speed =
            randf_range(config::MOB_MIN_SPEED as f64, config::MOB_MAX_SPEED as f64) as f32;

        let rotation = self.base().get_rotation();
        let mut velocity = Vector3::FORWARD * random_speed;
        velocity = Vector3::rotated(velocity, Vector3::UP, rotation.y);
        self.base_mut().set_velocity(velocity);

        let mut ap_ref: Gd<AnimationPlayer> = self.base().get_node_as("AnimationPlayer");
        ap_ref.set_speed_scale(random_speed / config::MOB_MIN_SPEED);
    }

    #[func]
    fn squash(&mut self) {
        self.base_mut().emit_signal("squashed".into(), &[]);
        self.base_mut().queue_free();
    }
}

impl Mob {}
