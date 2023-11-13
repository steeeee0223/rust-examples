pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("🚨 The secret number should lie between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("🎯 Exceptions - Define Custom Validation Type");

    let num: Guess = Guess::new(32);

    println!("💡 Guess value: {}", num.value);

    let num: Guess = Guess::new(125);

    println!("💡 Guess value: {}", num.value);
}
