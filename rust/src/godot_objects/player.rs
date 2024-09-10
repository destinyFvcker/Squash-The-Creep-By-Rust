use std::f32::consts::PI;

use crate::config;
use godot::classes::AnimationPlayer;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    #[export] // use #[export] to export fields to godot editor
    speed: f32,
    #[export]
    jump_impulse: f32,
    #[export]
    bounce_impulse: f32,
    #[export]
    fall_acceleration: f32,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    /// This is the constructor that must be overrided!
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: config::PLAYER_INIT_SPEED,
            jump_impulse: config::PLAYER_INIT_JUMP_IMPLULSE,
            bounce_impulse: config::PLAYER_INIT_BOUNCE_IMPULSE,
            fall_acceleration: config::PLYAER_INIT_FALL_ACCELERATION,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;
        let input_ref = Input::singleton();
        let mut ap_ref: Gd<AnimationPlayer> =
            self.base().get_node_as(NodePath::from("AnimationPlayer"));
        let mut velocity = self.base().get_velocity();

        if Input::is_action_pressed(&input_ref, StringName::from("move_right")) {
            direction.x += config::NORMAL_SPEED_PER_FRAME;
        }
        if Input::is_action_pressed(&input_ref, StringName::from("move_left")) {
            direction.x -= config::NORMAL_SPEED_PER_FRAME;
        }
        if Input::is_action_pressed(&input_ref, StringName::from("move_forward")) {
            direction.z -= config::NORMAL_SPEED_PER_FRAME;
        }
        if Input::is_action_pressed(&input_ref, StringName::from("move_back")) {
            direction.z += config::NORMAL_SPEED_PER_FRAME;
        }

        if direction != Vector3::ZERO {
            // In the lines below, we turn the character when moving and make the animation play faster.
            direction = direction.normalized();
            // Setting the basis property will affect the rotation of the node.
            self.base_mut()
                .set_basis(Basis::new_looking_at(direction, Vector3::UP, false));

            // The animation will be very weird when the character jumping up
            // if you open animation
            if !self.base().is_on_floor() {
                ap_ref.set_speed_scale(0.01)
            } else {
                ap_ref.set_speed_scale(6.0);
            }
        } else {
            ap_ref.set_speed_scale(1.5);
        }

        velocity.x = direction.x * self.speed;
        velocity.z = direction.z * self.speed;

        if self.base().is_on_floor()
            && Input::is_action_pressed(&input_ref, StringName::from("jump"))
        {
            velocity.y += self.jump_impulse;
        }

        velocity.y -= self.fall_acceleration * delta as f32;

        for index in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(index);
            // Actually I don't know why even we delect a slice collision, but the result of get() is still None!
            if let Some(collision) = collision {
                let mob = collision.get_collider();
                if let Some(mob) = mob {
                    let mut mob = mob.cast::<Node>();
                    if Node::is_in_group(&mob, StringName::from("mob")) {
                        if Vector3::UP.dot(collision.get_normal()) > 1.0f32.to_radians() {
                            mob.call(StringName::from("squash"), &[]);
                            velocity.y = self.bounce_impulse;
                            break;
                        }
                    }
                }
            }
        }

        let mut rotation = self.base().get_rotation();
        rotation.x = PI / 6f32 * velocity.y / self.jump_impulse;

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        self.base_mut().set_rotation(rotation);
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn die(&mut self) {
        self.base_mut().emit_signal("hit".into(), &[]);
        self.base_mut().queue_free();
    }
}

impl Player {}
