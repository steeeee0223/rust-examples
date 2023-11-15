fn add_fancy_hat() {
    println!("💡 Adding a fancy hat!");
}
fn remove_fancy_hat() {
    println!("💡 Removing a fancy hat!");
}
fn move_player(steps: u8) {
    println!("💡 Move player: {steps}");
}

fn role_dice(dice: u8) {
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn main() {
    println!("🎯 Match with default case!");

    let dice = 9;
    role_dice(dice);
}
