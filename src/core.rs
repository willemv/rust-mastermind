#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    CYAN,
    PURPLE,
}

pub enum Grade {
    Correct,
    Incorrect{correct_position: usize, correct_color: usize, wrong: usize},
}

pub fn grade(guess: &Vec<Color>, solution: &Vec<Color>) -> Grade {
    let mut correct_position = 0;
    let mut correct_color = 0;
    for (pos, color) in guess.iter().enumerate() {
        match solution.iter().position(|ref c| *c == color) {
            Some(position) => {
                if pos == position {
                    correct_position = correct_position + 1;
                } else {
                    correct_color = correct_color + 1;
                }
            }
            None => {}
        }
    }
    if correct_position == solution.len() {
        return Grade::Correct;
    }

    return Grade::Incorrect{
        correct_position,
        correct_color,
        wrong: solution.len() - correct_position - correct_color,
    };
}