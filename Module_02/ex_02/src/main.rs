mod car;
use car::Car;

fn main() {
    let car = Car::new("Fiat 500", 40);
    car.drive();

    let copy = car.clone();
    println!(
        "Copy name: '{}' and speed: {} km/h.",
        copy.name(),
        copy.speed(),
    );
    copy.drive();
}
