use bevy::prelude::*;

pub struct BankPlugin;

impl Plugin for BankPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bank>();
    }
}

#[derive(Resource)]
pub struct Bank {
    pub money: u32,
}

impl Bank {
    pub fn new(money: u32) -> Self {
        Self { money }
    }
}

impl Default for Bank {
    fn default() -> Self {
        Self { money: 10000 }
    }
}
