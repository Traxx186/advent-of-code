use aoc_common::input;

mod solutions;

fn main() {
    println!("🎄 Advent of Code 2023 Solutions 🎄");
    println!("      Justin van der Kruit       \n");
    println!("Type 'exit' to stop the program");

    for (i, item) in solutions::ALL.iter().enumerate() {
        println!("[{}] {}", i + 1, item.name());
    }

    loop {
        let run_day = input("\nDay > ").unwrap();
        if run_day == "exit" {
            break;
        }

        let run_day = match run_day.parse::<usize>() {
            Ok(day) => day,
            Err(_) => return println!("Thats no number :/")
        };

        if run_day > solutions::ALL.len() {
            println!("[*] The program only goes to day {}", solutions::ALL.len())
        } else {
            let part = input("Part (1 / 2) > ").unwrap();
            run(run_day, part);
        }
    }
}

fn run(day: usize, part: String) {
    let solution = solutions::ALL[day - 1];

    println!("[*] Running: {} ({})", solution.name(), part);

    let out = match part.as_str() {
        "1" => solution.part_1(),
        "2" => solution.part_2(),
        _ => return println!("[-] Invalid part")
    };

    println!("[+] OUT: {}", out)
}
