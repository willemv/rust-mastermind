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

pub fn grade(guess: &[Color], solution: &[Color]) -> Grade {
    let mut correct_position = 0;
    let mut correct_color = 0;
    for (pos, color) in guess.iter().enumerate() {
        if let Some(position) = solution.iter().position(|ref c| *c == color) {
            if pos == position {
                correct_position += 1;
            } else {
                correct_color += 1;
            }
        }
    }
    if correct_position == solution.len() {
        Grade::Correct
    } else {
        Grade::Incorrect {
            correct_position,
            correct_color,
            wrong: solution.len() - correct_position - correct_color,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Color::*;
    use super::*;

    #[test]
    fn grade_correct() {
        assert!(matches!(
            grade(
                &vec![RED, YELLOW, GREEN, BLUE],
                &vec![RED, YELLOW, GREEN, BLUE]
            ),
            Grade::Correct
        ));
    }

    #[test]
    fn grade_incorrect() {
        assert!(matches!(
            grade(
                &vec![RED, YELLOW, GREEN, BLUE],
                &vec![RED, YELLOW, GREEN, PURPLE]
            ),
            Grade::Incorrect {
                correct_position: 3,
                correct_color: 0,
                wrong: 1
            }
        ));
    }
}
