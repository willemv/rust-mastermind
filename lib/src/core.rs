#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Purple,
}

#[derive(Debug, Clone)]
pub enum Grade {
    Correct,
    Incorrect {
        correct_position: usize,
        correct_color: usize,
        wrong: usize,
    },
    Invalid(String),
}

pub fn grade(guess: &[Color], solution: &[Color]) -> Grade {
    if guess.len() != solution.len() {
        return Grade::Invalid("The guess should be as long as secret solution".to_string());
    }
    let mut pegs: Vec<Option<&Color>> = solution.iter().map(Some).collect();
    let mut correct_position = 0;
    let mut correct_color = 0;

    //first do a pass to find all guesses that have the correct color in the correct position
    for (pos, color) in guess.iter().enumerate() {
        let solution_peg = pegs[pos].unwrap();
        if solution_peg == color {
            correct_position += 1;
            pegs[pos] = None; //this peg should not be considered anymore in the next pass
        }
    }

    for color in guess.iter() {
        if let Some(position) = pegs.iter().position(|&c| matches!(c, Some(_)) && c.unwrap() == color) {
            correct_color += 1;
            pegs[position] = None;
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
            grade(&vec![Red, Yellow, Green, Blue], &vec![Red, Yellow, Green, Blue]),
            Grade::Correct
        ));
    }

    #[test]
    fn grade_incorrect() {
        assert!(matches!(
            grade(&vec![Red, Yellow, Green, Blue], &vec![Red, Yellow, Green, Purple]),
            Grade::Incorrect {
                correct_position: 3,
                correct_color: 0,
                wrong: 1
            }
        ));
    }

    #[test]
    fn grade_same_colors_1() {
        assert!(matches!(
            grade(&vec![Red, Red, Green, Green], &vec![Red, Red, Red, Green]),
            Grade::Incorrect {
                correct_position: 3,
                correct_color: 1,
                wrong: 0
            }
        ));
    }

    #[test]
    fn grade_same_colors_2() {
        assert!(matches!(
            grade(&vec![Red, Red, Green, Green], &vec![Red, Red, Red, Green]),
            Grade::Incorrect {
                correct_position: 3,
                correct_color: 1,
                wrong: 0
            }
        ));
    }

    #[test]
    fn grade_same_colors_3() {
        assert!(matches!(
            grade(&vec![Red, Red, Green, Green], &vec![Green, Red, Red, Yellow]),
            Grade::Incorrect {
                correct_position: 1,
                correct_color: 2,
                wrong: 1
            }
        ));
    }
}
