use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct MoveComponent {
    pub ticks_passed: i32,
    pub ticks_to_move: i32,
}

#[derive(Debug, Component)]
pub struct HPComponent {
    pub hp_current: f32,
    pub hp_max: f32,
}

impl HPComponent {
    /// damages this HPComponent and returns if it dit hit 0 HP. (dead)
    pub fn damage_is_dead(&mut self, damage: f32) -> bool {
        if damage >= self.hp_current {
            self.hp_current = 0.;
            true
        } else {
            self.hp_current -= damage;
            false
        }
    }

    pub(crate) fn new(hp_max: f32) -> Self {
        HPComponent {
            hp_current: hp_max,
            hp_max: hp_max,
        }
    }
}

#[derive(Debug, Component)]
pub struct ShootComponent {
    pub range: i32,
    pub damage: f32,

    pub ticks_to_fire: i32,
    pub ticks_passed: i32,
}

impl ShootComponent {
    /// processes one tick and returns if the component can shoot.
    pub fn can_shoot_tick(&mut self) -> bool {
        self.ticks_passed += 1;
        return self.ticks_passed >= self.ticks_to_fire;
    }

    /// resets the tick timer so a shoot just happens
    pub fn notify_shoot(&mut self) {
        self.ticks_passed = 0;
    }
}

#[derive(Debug, Component)]
pub struct Tower {
    pub range: i32,
    pub damage: f32,

    pub ticks_to_fire: i32,
    pub ticks_passed: i32,
}

#[derive(Debug, Component)]
pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
}
