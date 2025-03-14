use bevy::prelude::*;

pub struct BankPlugin;

impl Plugin for BankPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bank>().add_observer(adjust_funds);
    }
}

#[derive(Event)]
pub struct AdjustFundsEv(pub i32);

#[derive(Resource)]
pub struct Bank {
    pub money: i32,
}

impl Bank {
    pub fn new(money: i32) -> Self {
        Self { money }
    }

    pub fn add(&mut self, amount: i32) {
        self.money += amount;
    }

    pub fn remove(&mut self, amount: i32) {
        self.money -= amount;
    }
}

impl Default for Bank {
    fn default() -> Self {
        Self { money: 10000 }
    }
}

fn adjust_funds(trigger: Trigger<AdjustFundsEv>, mut bank: ResMut<Bank>) {
    let amount = trigger.0;
    bank.money += amount;
}
