const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
    // const READY_AMOUNT:i32 = 1;
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;

    let (missiles ,ready):(i32,i32) = (STARTING_MISSILES,READY_AMOUNT);
    println!("Firing {} of my {} missiles",ready,missiles);
    // missiles = missiles - ready;
    println!("{} missiles left",missiles-ready);

    // const STARTING_MISSILES:i32 = 8;
    // const READY_AMOUNT:i32 = 2;

}
