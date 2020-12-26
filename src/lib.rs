pub mod guessing_game {
    use std::io;
    use rand::Rng;

    #[derive(Debug, PartialEq)]
    enum GuessResult {
        Nan,
        Lower,
        Higher,
        GotIt
    }

    fn evaluate_guess(secret_num:i32, guess: &str) -> GuessResult {
        if let Ok(guess) = guess.trim().parse::<i32>() {
            if guess < secret_num {
                GuessResult::Higher
            } else if guess > secret_num {
                GuessResult::Lower
            } else {
                GuessResult::GotIt
            }
        } else {
            GuessResult::Nan
        }
    }

    pub struct Game {
        secret_num: i32,
    }

    impl Game {
        pub fn new(secret:i32) -> Game {
            Game {
                secret_num: secret,
                }
            }


        pub fn random() -> i32 {
            rand::thread_rng().gen_range(1, 101)
        }
    
        pub fn run(&self) {
            let _ = self.run_with_buffers(
                &mut io::stdin().lock(), &mut io::stdout());
        }

        pub fn run_with_buffers(&self, reader: &mut dyn io::BufRead,
                   writer: &mut dyn io::Write) -> io::Result<()> {
            let mut guessed = false;

            while !guessed {
                writeln!(writer, "type your guess:")?;
                let mut guess = String::new();
                reader.
                    read_line(&mut guess)?;
                    //.expect("failed to read line")?;

                writeln!(writer, "you guessed {}", guess)?;
                match evaluate_guess(self.secret_num, &guess) {
                    GuessResult::Higher => {
                        writeln!(writer, "higher!")?;
                    },
                    GuessResult::Lower => {
                        writeln!(writer, "lower!")?;
                    },
                    GuessResult::GotIt => {
                        writeln!(writer, "you guessed correctly! Well done!")?;
                        guessed = true;
                    },
                    GuessResult::Nan => {
                        writeln!(writer,
                            "that's not something I can read as a number")?;
                    }
                };
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn correct() {
            let r = evaluate_guess(3, "3");
            assert_eq!(r, GuessResult::GotIt);
        }

        #[test]
        fn nan() {
            let r = evaluate_guess(3, "foo");
            assert_eq!(r, GuessResult::Nan);
        }

        #[test]
        fn lower() {
            let r = evaluate_guess(5, "77");
            assert_eq!(r, GuessResult::Lower);
        }
        
        #[test]
        fn higher() {
            let r = evaluate_guess(65, "64");
            assert_eq!(r, GuessResult::Higher);
        }

        #[test]
        fn line_ending() {
            let r = evaluate_guess(65, "65\n");
            assert_eq!(r, GuessResult::GotIt);
        }
        
        #[test]
        fn rng() {
            for _ in 0..100 {
                let r = Game::random();
                assert!(r > 0 && r < 101);
            }
        }

        #[test]
        fn game() {
            let guesses = "42\n56\n50";
            let game = Game::new(50);
            let mut output = Vec::<u8>::new();
            let r = game.run_with_buffers(
                &mut guesses.as_bytes(), &mut output);
            assert!(r.is_ok());
        }
    }
}

