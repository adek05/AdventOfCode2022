use crate::utils::read_lines;

pub fn run() {
    let lines = read_lines("in/day10.in").unwrap();
    let mut register = 1;

    let mut register_values = vec![];
    register_values.push(1);

    for line in lines {
        let l = line.unwrap();
        scan!(&l;
          ("noop") => {
            register_values.push(register);
          },
          ("addx", let delta: i32) => {
            register_values.push(register);
            register_values.push(register);
            register += delta;
          },
        )
        .unwrap();
    }

    let part1 = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|idx| *idx as i64 * register_values[*idx] as i64)
        .sum::<i64>();
    println!("Day 10, part 1 {}", part1);

    for y in 0..6 {
        for x in 0..40 {
            let cycle = y * 40 + x;
            let sprite_pos = register_values[cycle + 1];
            if (cycle as i32 % 40 - sprite_pos).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
