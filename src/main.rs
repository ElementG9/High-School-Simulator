use high_school_simulator::hs_sim::Simulator;
fn main() {
    let mut sim = Simulator::new();
    println!("Welcome to High School Simulator 2020!");
    Simulator::print_information();
    loop {
        sim.take_turn();
    }
    println!("How did you get here?");
}
