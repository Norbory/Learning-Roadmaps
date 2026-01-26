////////// Structs
struct RedFox {
    enemy: bool,
    life: u8,
}

impl RedFox {
    // associated function
    fn new() -> Self {
        Self {
            enemy: true,
            life: 5,
        }
    }
    // method
    fn some_method(&self) {
        println!("Enemy: {}, Life: {}", self.enemy, self.life);
    }
    fn move(self) ...
    fn borrow(&self) ...
    fn mut_borrow(&mut self) ...
}

// Instances
let fox = RedFox {
    enemy: true,
    life: 5,
};
let foxy = RedFox::new();
let life_foxy = foxy.life;
foxy.enemy = false;
foxy.some_method();


////////// Traits
trait Noisy {
    fn get_noise(&self) -> &str;
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {"Beep"}
}

fn main() {
    print_noise(5_u8);
}

// Copy
