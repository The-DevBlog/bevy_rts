use bevy::prelude::*;

use crate::cmd_interface::components::BankTxt;

pub struct BankPlugin;

impl Plugin for BankPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bank>()
            .add_systems(Update, update_bank_funds)
            .add_observer(adjust_funds);
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

fn update_bank_funds(
    time: Res<Time>,
    mut bank: ResMut<Bank>,
    mut bank_txt: Query<&mut Text, With<BankTxt>>,
) {
    if bank.funds == bank.displayed_funds {
        return;
    }

    let target = bank.funds;
    let speed = 1250.0; // units per second
    let diff = (target - bank.displayed_funds) as f32;
    let step = speed * time.delta_secs();

    if diff.abs() < step {
        bank.displayed_funds = target;
    } else if diff > 0.0 {
        bank.displayed_funds += step as i32;
    } else {
        bank.displayed_funds -= step as i32;
    }

    let mut text = bank_txt.single_mut();
    text.0 = format!("${}", bank.displayed_funds);
}
