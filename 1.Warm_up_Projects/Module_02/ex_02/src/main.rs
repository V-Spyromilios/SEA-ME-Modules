mod car;
use car::Car;

fn main() {
    let car = Car::new("Fiat 500", 40);
    car.drive();

    let copy = car.clone();
    println!("{:?}", copy);
    copy.drive();
}
