fn main() {
    let mut all_elf_calories = parse_input(
        std::io::stdin()
            .lines()
            .map(|lr| lr.expect("stdin read error")),
    );

    all_elf_calories.sort_unstable();

    dbg!("part1", all_elf_calories.iter().rev().next());
    dbg!("part2", all_elf_calories.iter().rev().take(3).sum::<u64>());
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<u64> {
    let mut res = Vec::new();
    let mut current_elf = 0;

    for line in lines.map(|l| l.parse::<u64>().ok()) {
        match line {
            Some(cal_count) => current_elf += cal_count,
            None => {
                res.push(current_elf);
                current_elf = 0;
            }
        }
    }

    res
}
