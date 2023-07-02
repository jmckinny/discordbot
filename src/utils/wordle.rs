use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    solution: String,
    guess_left: u8,
    game_won: bool,
}

#[derive(Debug)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

pub type LetterScore = (char, Correctness);

#[derive(Debug)]
pub struct GuessScore {
    data: Vec<LetterScore>,
}

impl Game {
    pub fn new(solution: &str) -> Self {
        Game {
            solution: solution.to_string(),
            guess_left: 6,
            game_won: false,
        }
    }

    pub async fn guess(&mut self, guess: &str) -> Result<GuessScore, &str> {
        if !is_valid_word(guess).await {
            return Err("Invalid guess");
        }
        self.guess_left -= 1;

        let mut correctness = Vec::new();
        let mut letter_freq: HashMap<char, u8> = HashMap::new();

        for letter in self.solution.chars() {
            if let Some(freq) = letter_freq.get_mut(&letter) {
                *freq += 1;
            } else {
                letter_freq.insert(letter, 1);
            }
        }

        for (i, letter) in guess.chars().enumerate() {
            if let Some(freq) = letter_freq.get_mut(&letter) {
                if *freq > 0 {
                    if self.solution.chars().nth(i).unwrap() == letter {
                        correctness.push((letter, Correctness::Correct));
                        *freq = freq.checked_sub(1).unwrap_or(0);
                    } else if self.later_correct(letter, i) > 0
                        && guess.chars().skip(i + 1).filter(|c| *c == letter).count() > 0
                    {
                        correctness.push((letter, Correctness::Wrong));
                    } else {
                        correctness.push((letter, Correctness::Misplaced));
                        *freq = freq.checked_sub(1).unwrap_or(0);
                    }
                } else {
                    correctness.push((letter, Correctness::Wrong));
                }
            } else {
                correctness.push((letter, Correctness::Wrong));
            }
        }

        if self.solution == guess {
            self.game_won = true;
        }

        Ok(GuessScore { data: correctness })
    }

    fn later_correct(&self, letter: char, index: usize) -> usize {
        let mut result = 0;
        for c in self.solution.chars().skip(index + 1) {
            if c == letter {
                result += 1;
            }
        }
        result
    }

    pub fn is_game_over(&self) -> bool {
        self.guess_left == 0 || self.game_won
    }

    pub fn game_won(&self) -> bool {
        self.game_won
    }

    pub fn guesses_left(&self) -> u8 {
        self.guess_left
    }
}

async fn is_valid_word(word: &str) -> bool {
    let words = tokio::fs::read_to_string("wordlist.txt")
        .await
        .expect("Failed to read word list");
    for w in words.lines() {
        if word == w {
            return true;
        }
    }
    false
}

impl std::fmt::Display for GuessScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, correctness) in self.data.iter() {
            match correctness {
                Correctness::Correct => write!(f, "🟩")?,
                Correctness::Misplaced => write!(f, "🟨")?,
                Correctness::Wrong => write!(f, "🟥")?,
            }
        }
        Ok(())
    }
}
