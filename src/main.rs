use std::io;

fn main() {
    let spec_watts = 1100;
    let true_watts = 700;
    let spec_time = get_time();
    let original_energy = calculate_energy(spec_watts, spec_time);
    let true_time = calculate_seconds(original_energy, true_watts);
    let true_mins = (true_time as f64 / 60.0).ceil() as i32;
    let true_sec = (true_time as f64 / 60.0).floor() as i32;
    println!("Microwave for {true_mins} minutes and {true_sec} seconds.")
}

fn get_time() -> i32 {
    println!("How long does the package say to nuke it for? (Use minutes)");
    let mut time_string = String::new();
    io::stdin().read_line(&mut time_string).expect("Could not read input!");
    let minutes: i32 = time_string.trim().parse().expect("Input was not a number!");
    minutes * 60
}

fn calculate_energy(wattage: i32, seconds: i32) -> i32 {
    wattage * seconds
}

fn calculate_seconds(energy: i32, power: i32) -> i32 {
    energy / power
}
