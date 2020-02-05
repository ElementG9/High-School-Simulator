extern crate rand;
pub mod hs_sim {
    use rand::{thread_rng, Rng};
    pub struct Simulator {
        done: usize,
        stress: usize,
        missing: usize,
        fatigue: usize,
        grade: usize,
    }
    impl Simulator {
        // Struct methods
        pub fn new() -> Simulator {
            Simulator {
                done: 0,
                missing: 0,
                stress: 0,
                fatigue: 0,
                grade: 100,
            }
        }
        pub fn work_legit(&mut self) {
            println!("Do work: Getting work done!");
            self.done += 1;
            self.stress += 2;
            self.missing += 2;
            self.fatigue += 1;
            if self.grade >= 2 {
                self.grade -= 2;
            }
            self.check_status();
        }
        pub fn work_cram(&mut self) {
            if self.fatigue > 140 {
                println!("Cram work: You are too fatigued to cram work!");
            } else if self.stress > 240 {
                println!("Cram work: You are too stressed to work!");
            } else if self.missing < 1 {
                println!("Cram work: You don't need to cram!");
            } else {
                println!("Cram work: Cramming lots of work");
                self.done += 2;
                self.stress += 10;
                if self.missing >= 1 {
                    self.missing -= 1;
                }
                self.fatigue += 10;
                self.grade += 3;
                self.check_status();
            }
        }
        pub fn sleep(&mut self) {
            if self.fatigue == 0 || self.stress == 0 {
                println!("Sleep: You don't need to sleep!");
            } else {
                println!("Sleep: Sleeping the day away...");
                self.done += 6;
                if self.stress >= 5 {
                    self.stress -= 5;
                }
                if self.missing >= 5 {
                    self.missing -= 5;
                }
                self.fatigue += 1;
                self.grade += 8;
                self.check_status();
            }
        }
        pub fn cheat(&mut self) {
            let mut rng = thread_rng();
            if rng.gen_ratio(1, 4) {
                // Caught
                println!("Cheat: You got caught cheating!");
                Simulator::end_game();
            } else {
                // Not caught
                println!("Cheat: Copying others work...");
                self.done += 6;
                if self.stress >= 5 {
                    self.stress -= 5;
                }
                if self.missing >= 5 {
                    self.missing -= 5;
                }
                self.fatigue += 1;
                self.grade += 8;
                self.check_status();
            }
        }
        pub fn to_string(&self) -> String {
            format!("Status:\n\tAssignments Done: {}\n\tAssignments Missing: {}\n\tStress: {}\n\tFatigue: {}\n\tGrade: {}%",
                self.done,
                self.missing,
                self.stress,
                self.fatigue,
                self.grade
            )
        }
        pub fn check_status(&self) {
            if self.fatigue > 150 || self.stress > 250 || self.missing > 250 {
                println!("You are too worn out to keep working!");
                Simulator::end_game();
            } else if self.grade < 50 {
                println!("You failed school!");
                Simulator::end_game();
            }
        }
        pub fn take_turn(&mut self) {
            let mut got_valid_response = false;
            while !got_valid_response {
                println!("Choose:\n\t1. Do work\n\t2. Cram work\n\t3. Sleep\n\t4. Cheat\n\t5. Exit");
                let mut choice = String::new();
                let _b = std::io::stdin().read_line(&mut choice);
                match choice.trim() {
                    "1" => {
                        got_valid_response = true;
                        self.work_legit();
                    },
                    "2" => {
                        got_valid_response = true;
                        self.work_cram();
                    },
                    "3" => {
                        got_valid_response = true;
                        self.sleep();
                    },
                    "4" => {
                        got_valid_response = true;
                        self.cheat();
                    },
                    "5" => {
                        got_valid_response = true;
                        Simulator::end_game();
                    },
                    _ => {
                        got_valid_response = false;
                        println!("Choose: Please choose 1, 2, 3, or 4.");
                    }
                };
                println!("{}", self.to_string());
            }
        }
        // Associated functions
        pub fn print_information() {
            let mut s = String::new();
            s.push_str("Information:\n");
            s.push_str("\tYou lose if grade < 50%, fatigue > 150, stress > 250 or you miss 250 assignments or more.\n");
            s.push_str("\tBalance doing work, cramming, and sleeping to win.\n");
            s.push_str("\tCheat only when you have to.\n");
            print!("{}", s);
        }
        pub fn end_game() {
            println!("Thanks for playing High School Simulator 2020!");
            std::process::exit(0);
        }
    }
}
