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
    pub funds: i32,
    pub displayed_funds: i32,
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            funds: 10000,
            displayed_funds: 0,
        }
    }
}

fn adjust_funds(trigger: Trigger<AdjustFundsEv>, mut bank: ResMut<Bank>) {
    let adjustment = trigger.0;
    bank.funds += adjustment;
}
