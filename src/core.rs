#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    CYAN,
    PURPLE,
}

#[derive(Debug)]
pub enum Grade {
    Correct,
    Incorrect {
        correct_position: usize,
        correct_color: usize,
        wrong: usize,
    },
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

    return Grade::Incorrect {
        correct_position,
        correct_color,
        wrong: solution.len() - correct_position - correct_color,
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Color::*;

    #[test]
    fn grade_correct() {
        assert!(matches!(
            grade(&vec![RED, YELLOW, GREEN, BLUE], &vec![RED, YELLOW, GREEN, BLUE]), 
            Grade::Correct
        ));
    }

    #[test]
    fn grade_incorrect() {
        assert!(matches!(
            grade(&vec![RED, YELLOW, GREEN, BLUE], &vec![RED, YELLOW, GREEN, PURPLE]), 
            Grade::Incorrect{correct_position: 3, correct_color: 0, wrong: 1}
        ));
    }
}