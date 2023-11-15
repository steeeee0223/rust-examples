pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("🚀 Breaking news! {}", item.summarize());
}
