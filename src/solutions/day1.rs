pub fn run(data: &String) -> String {
    let mut current_total = 0;
    let mut elf_calories: Vec<i32> = Vec::new();

    // First read the number of calories that each elf is carying
    for line in data.lines() {
        if line.len() > 0 {
            // Convert the current line to an int, and add it to current_total
            current_total += match i32::from_str_radix(line, 10) {
                Ok(c) => c,
                Err(e) => {
                    eprint!(
                        "Error parsing number of calories: {}. {}",
                        line,
                        e.to_string()
                    );
                    0
                }
            };
        } else {
            elf_calories.push(current_total);
            current_total = 0;
        }
    }

    // Now find the highest number of calories in the vector
    let max_calories: i32 = match elf_calories.iter().max() {
        Some(m) => *m,
        None => {
            eprint!("No elves found =(");
            0
        }
    };

    // Get the sum of the top 3 elves
    elf_calories.sort();
    elf_calories.reverse();
    let largest_3 = elf_calories[0..3].to_vec();
    let largest_3_calories: i32 = largest_3.iter().sum();

    return format!(
        "Most calories by single elf: {}\nTop 3 elves worth of calories: {}",
        max_calories, largest_3_calories
    );
}
