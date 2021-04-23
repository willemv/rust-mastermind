use crate::core::*;

#[derive(Clone, Debug)]
pub struct GameState {
    pub secret: Vec<Color>,
    pub max_attempts: usize,
    pub attempts: Vec<Attempt>,
    pub state: State,
}

#[derive(Clone, Debug)]
pub struct Attempt {
    pub guess: Vec<Color>,
    pub grade: Grade,
}

#[derive(Clone, Debug)]
pub enum State {
    Finished(bool),
    AwaitingAttempt(bool), //true if the last attempt was valid, false otherwise
}

/// starts a new game, for the given secret
pub fn start(secret: &[Color], max_attempts: usize) -> GameState {
    GameState {
        secret: Vec::from(secret),
        max_attempts,
        attempts: vec![],
        state: State::AwaitingAttempt(true),
    }
}

impl GameState {
    fn for_attempt(&self, attempt: Attempt, state: State) -> GameState {
        GameState {
            secret: self.secret.clone(),
            max_attempts: self.max_attempts,
            attempts: clone_and_append(&self.attempts, attempt),
            state,
        }
    }

    fn with_state(&self, state: State) -> GameState {
        GameState {
            secret: self.secret.clone(),
            max_attempts: self.max_attempts,
            attempts: self.attempts.clone(),
            state,
        }
    }

    pub fn guess_count(&self) -> usize {
        self.attempts.len()
    }
}

pub fn attempt(current_state: &GameState, guess: Vec<Color>) -> GameState {
    let grade = grade(&guess, &current_state.secret);
    match grade {
        Grade::Correct => current_state.for_attempt(Attempt { guess, grade }, State::Finished(true)),
        //invalid attempts are not counted
        Grade::Invalid(_) => current_state.with_state(State::AwaitingAttempt(false)),
        Grade::Incorrect {
            correct_position: _,
            correct_color: _,
            wrong: _,
        } => {
            if current_state.attempts.len() + 1 >= current_state.max_attempts {
                current_state.for_attempt(Attempt { guess, grade }, State::Finished(false))
            } else {
                current_state.for_attempt(Attempt { guess, grade }, State::AwaitingAttempt(true))
            }
        }
    }
}

fn clone_and_append(attempts: &[Attempt], attempt: Attempt) -> Vec<Attempt> {
    let mut attempts = attempts.to_owned();
    attempts.push(attempt);
    attempts
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::Color::*;

    #[test]
    fn test_correct_attempt() {
        let game = start(&[Red, Yellow, Green, Blue], 6);
        let result = attempt(&game, vec![Red, Yellow, Green, Blue]);
        assert!(matches!(result.state, State::Finished(true)));
        assert!(game.attempts.is_empty());
    }

    #[test]
    fn happy_path() {
        let state = start(&[Red, Yellow, Green, Blue], 4);
        let state = attempt(&state, vec![Red, Purple, Green, Blue]);

        assert!(matches!(state.state, State::AwaitingAttempt(true)));
        assert!(state.attempts.len() == 1);
    }
}
