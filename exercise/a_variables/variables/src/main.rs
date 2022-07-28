const STARTING_MISSILES: i8 = 8;
const READY_AMMOUNT: i8 = 2;

fn main() {
    let _ready_amount = 1;
    let (missiles ,ready): (i32, i32) =
        (STARTING_MISSILES.into(), READY_AMMOUNT.into());

    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
