use crate::ui::Input;
use core::fmt::{Result as FmtResult, Write};

pub struct InputRecorder<'output, Output: Write> {
    output: &'output mut Output,
    last_input: Option<(Input, usize)>,
}

impl<'output, Output: Write> InputRecorder<'output, Output> {
    pub fn new(output: &'output mut Output) -> Self {
        InputRecorder {
            output,
            last_input: None,
        }
    }

    pub fn record_input(&mut self, input: Input) -> FmtResult {
        if let Some((last_input, count)) = self.last_input {
            if input == last_input {
                self.last_input = Some((input, count + 1));
                return Ok(());
            }

            self.flush()?;
        }
        self.last_input = Some((input, 1));
        Ok(())
    }

    pub fn flush(&mut self) -> FmtResult {
        if let Some((last_input, count)) = self.last_input {
            self.last_input = None;
            let mangled = match last_input {
                Input::KeyUp => '↑',
                Input::KeyDown => '↓',
                Input::Enter => '⏎',
                Input::Other => '.',
            };
            if count == 1 {
                self.output.write_char(mangled)
            } else {
                write!(self.output, "{}{}", count, mangled)
            }
        } else {
            Ok(())
        }
    }

    pub fn output(&self) -> &Output {
        self.output
    }
}

#[derive(Debug)]
pub enum InputRecordingParserError {
    ParseInt {
        grapheme: usize,
        error: core::num::ParseIntError,
    },
    UnknownCharacter {
        grapheme: usize,
    },
    UnexpectedEOF,
    Interrupted,
}

pub struct InputRecordingParser<'input> {
    input: &'input str,
}

impl<'input> InputRecordingParser<'input> {
    pub fn new(input: &'input str) -> Self {
        InputRecordingParser { input }
    }

    fn demangle_input(
        &self,
        grapheme: usize,
        c: char,
    ) -> Result<Input, InputRecordingParserError> {
        match c {
            '↑' => Ok(Input::KeyUp),
            '↓' => Ok(Input::KeyDown),
            '⏎' => Ok(Input::Enter),
            '.' => Ok(Input::Other),
            _ => Err(InputRecordingParserError::UnknownCharacter { grapheme }),
        }
    }

    pub fn parse_all<F: FnMut(Input) -> bool>(
        &mut self,
        mut into: F,
    ) -> Result<(), InputRecordingParserError> {
        let mut number_start = None;
        let mut grapheme = 0usize;
        for (i, c) in self.input.char_indices() {
            match number_start {
                None => {
                    if c.is_ascii_digit() {
                        number_start = Some(i);
                    } else {
                        let input = self.demangle_input(grapheme, c)?;
                        if !into(input) {
                            return Err(InputRecordingParserError::Interrupted);
                        }
                    }
                }
                Some(start_position) => {
                    if !c.is_ascii_digit() {
                        // Встрелили первый символ, не являющийся цифрой.
                        // Парсим число.
                        let parsed_number =
                            match self.input[start_position..i].parse::<usize>() {
                                Ok(number) => number,
                                Err(error) => {
                                    return Err(InputRecordingParserError::ParseInt {
                                        grapheme,
                                        error,
                                    })
                                }
                            };
                        number_start = None;
                        let input = self.demangle_input(grapheme, c)?;
                        for _ in 0..parsed_number {
                            if !into(input) {
                                return Err(InputRecordingParserError::Interrupted);
                            }
                        }
                    }
                }
            }
            grapheme += 1;
        }

        if number_start.is_none() {
            Ok(())
        } else {
            Err(InputRecordingParserError::UnexpectedEOF)
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording() -> FmtResult {
        let mut output = String::new();
        let mut recorder = InputRecorder::new(&mut output);

        recorder.record_input(Input::KeyDown)?;
        recorder.record_input(Input::Enter)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::Other)?;
        recorder.record_input(Input::KeyDown)?;
        recorder.record_input(Input::KeyDown)?;
        recorder.record_input(Input::Enter)?;
        recorder.record_input(Input::Enter)?;
        recorder.record_input(Input::Enter)?;
        recorder.record_input(Input::Enter)?;
        recorder.record_input(Input::Enter)?;
        recorder.record_input(Input::Other)?;
        recorder.record_input(Input::Other)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyUp)?;
        recorder.record_input(Input::KeyDown)?;
        recorder.flush()?;

        assert_eq!(output, "↓⏎↑.2↓5⏎2.12↑↓");

        Ok(())
    }

    #[test]
    fn test_successful_parsing() -> Result<(), InputRecordingParserError> {
        let input = "↓⏎↑.2↓5⏎2.12↑↓";
        let mut parser = InputRecordingParser::new(&input);
        let mut parsed_input = Vec::new();

        parser.parse_all(|input| {
            parsed_input.push(input);
            true
        })?;

        assert_eq!(
            parsed_input,
            [
                Input::KeyDown,
                Input::Enter,
                Input::KeyUp,
                Input::Other,
                Input::KeyDown,
                Input::KeyDown,
                Input::Enter,
                Input::Enter,
                Input::Enter,
                Input::Enter,
                Input::Enter,
                Input::Other,
                Input::Other,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyUp,
                Input::KeyDown,
            ]
        );

        Ok(())
    }

    #[test]
    fn test_parsing_unexpected_eof() {
        let input = "↓↓2⏎13";
        let mut parser = InputRecordingParser::new(&input);
        let mut parsed_input = Vec::new();

        let result = parser.parse_all(|input| {
            parsed_input.push(input);
            true
        });

        assert_eq!(
            parsed_input,
            [Input::KeyDown, Input::KeyDown, Input::Enter, Input::Enter]
        );

        assert!(matches!(
            result,
            Err(InputRecordingParserError::UnexpectedEOF)
        ));
    }

    #[test]
    fn test_parsing_unknown_character() {
        let input = "↓2⏎3!⏎";
        let mut parser = InputRecordingParser::new(&input);
        let mut parsed_input = Vec::new();

        let result = parser.parse_all(|input| {
            parsed_input.push(input);
            true
        });

        assert_eq!(parsed_input, [Input::KeyDown, Input::Enter, Input::Enter]);

        assert!(matches!(
            result,
            Err(InputRecordingParserError::UnknownCharacter { grapheme: 4 })
        ));
    }
}
