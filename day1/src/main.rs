use std::fs;

fn main() {
    // read file into string
    let input: String = fs::read_to_string("src/input.txt").unwrap();

    // PART 1

    // split by lines
    let lines = input.split("\n");

    let mut max_cal = 0;
    let mut elf_cal = 0;
    for line in lines {
        if line == "" {
            // record elf calories if max
            if elf_cal > max_cal {
                max_cal = elf_cal;
                println!("New max cal {}", max_cal)
            }
            // reset elf calories
            elf_cal = 0;
            continue;
        }

        // parse into int
        let cal: i32 = line.parse::<i32>().expect("error parsing line");
        elf_cal += cal;
    }

    println!("\n\n{}", max_cal);

    // PART 2

    // split by lines
    let lines = input.split("\n");

    let mut max_cals: [i32; 3] = [0, 0, 0];
    let mut elf_cal = 0;
    for line in lines {
        if line == "" {
            // find if larger than the smallest of the three elfs in max_cals
            // we first sort so smallest gets replaced first
            max_cals.sort();
            for max_cal in max_cals.into_iter().enumerate() {
                let (i, cal): (usize, i32) = max_cal;
                if elf_cal > cal {
                    max_cals[i] = elf_cal;
                    println!("New max cals {:?}", max_cals);
                    break;
                }
            }
            // reset elf calories
            elf_cal = 0;
            continue;
        }

        // parse into int
        let cal: i32 = line.parse::<i32>().expect("error parsing line");
        elf_cal += cal;
    }

    println!("\n\n{}", max_cals[0] + max_cals[1] + max_cals[2]);
}
