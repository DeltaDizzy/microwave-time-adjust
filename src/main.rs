extern crate uom;

use std::io;
use uom::si::{f64::*, power, time};

fn main() {
    let spec_watts = Power::new::<power::watt>(1100.0);
    let true_watts = Power::new::<power::watt>(1100.0);
    let spec_time = get_input_time();
    let original_energy = calculate_energy(spec_watts, spec_time);
    let true_time = split_time(calculate_seconds(original_energy, true_watts));
    println!("Microwave for {} minutes and {} seconds", true_time.0, true_time.1);
}

fn split_time(time: Time) -> (i32, i32) {
    let start_seconds = time.get::<time::second>() as i32;
    let minutes = start_seconds as f64 / 60.0;
    (minutes.floor() as i32, start_seconds % 60)
}

fn get_input_time() -> Time {
    println!("How long does the package say to nuke it for? (Use minutes)");
    let mut time_string = String::new();
    io::stdin().read_line(&mut time_string).expect("Could not read input!");
    
    let minutes = Time::new::<time::minute>(time_string.trim().parse().expect("Expected an integer number"));
    minutes
}

fn calculate_energy(wattage: Power, seconds: Time) -> Energy {
    wattage * seconds
}

fn calculate_seconds(energy: Energy, power: Power) -> Time {
    energy / power
}

#[cfg(test)]
mod test {

    use uom::si::energy;

    use super::*;

    #[test]
    fn energy_test() {

        assert_eq!(calculate_energy(Power::new::<power::watt>(2.0), Time::new::<time::second>(2.0)), Energy::new::<energy::joule>(4.0));
    }

    #[test]
    fn time_test() {

        assert_eq!(calculate_seconds(Energy::new::<energy::joule>(25.0), Power::new::<power::watt>(5.0)), Time::new::<time::second>(5.0));
    }

    #[test]
    fn split_test() {
        let start_seconds = Time::new::<time::second>(125.0);
        let split_result = split_time(start_seconds);
        assert_eq!((2, 5), split_result);
    }
}
