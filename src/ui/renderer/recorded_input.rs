use super::*;

#[derive(Debug, Eq, PartialEq)]
pub enum RecordedInputRendererError {
    ExpectedMoreInput,
}

/// This renderer does nothing except replaying the input values passed to it to whoever
/// calls its `getch` method.
///
/// This is useful for fuzzing the game logic.
pub struct RecordedInputRenderer<I: Iterator<Item = Input>> {
    iterator: I,
    next: Option<I::Item>,
}

impl<I: Iterator<Item = Input>> RecordedInputRenderer<I> {
    pub fn new(mut input: I) -> RecordedInputRenderer<I> {
        let next = input.next();
        RecordedInputRenderer {
            iterator: input,
            next,
        }
    }
}

macro_rules! error_if_finished {
    ($renderer:expr) => {{
        if $renderer.next.is_none() {
            Err(RecordedInputRendererError::ExpectedMoreInput)
        } else {
            Ok(Default::default())
        }
    }};
}

impl<I: Iterator<Item = Input>> Renderer for RecordedInputRenderer<I> {
    type Error = RecordedInputRendererError;

    fn clear_screen(&mut self) -> Result<(), Self::Error> {
        error_if_finished!(self)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        error_if_finished!(self)
    }

    fn write_str(&mut self, _s: &str) -> Result<(), Self::Error> {
        error_if_finished!(self)
    }

    fn move_cursor_to(&mut self, _line: i32, _column: i32) -> Result<(), Self::Error> {
        error_if_finished!(self)
    }

    fn get_cursor_position(&mut self) -> Result<(i32, i32), Self::Error> {
        error_if_finished!(self)
    }

    fn set_color(
        &mut self,
        _foreground: Color,
        _background: Color,
    ) -> Result<(), Self::Error> {
        error_if_finished!(self)
    }

    fn getch(&mut self) -> Result<Input, Self::Error> {
        if let Some(input) = self.next {
            self.next = self.iterator.next();
            Ok(input)
        } else {
            Err(RecordedInputRendererError::ExpectedMoreInput)
        }
    }

    fn sleep_ms(&mut self, _ms: Milliseconds) -> Result<(), Self::Error> {
        error_if_finished!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Input::*;
    use RecordedInputRendererError::*;

    #[test]
    fn test_getch() {
        let input = [KeyDown, Enter, KeyUp, Other, Enter, KeyUp, KeyDown];
        let mut renderer = RecordedInputRenderer::new(input.iter().cloned());

        for &event in input.iter() {
            assert_eq!(renderer.clear_screen(), Ok(()));
            assert_eq!(renderer.flush(), Ok(()));
            assert_eq!(renderer.write_str("hello"), Ok(()));
            assert_eq!(renderer.move_cursor_to(1, 2), Ok(()));
            assert_eq!(renderer.get_cursor_position(), Ok((0, 0)));
            assert_eq!(renderer.set_color(Color::WhiteBright, Color::Red), Ok(()));
            assert_eq!(renderer.sleep_ms(Milliseconds(0)), Ok(()));
            assert_eq!(renderer.getch(), Ok(event));
        }

        assert_eq!(renderer.clear_screen(), Err(ExpectedMoreInput));
        assert_eq!(renderer.flush(), Err(ExpectedMoreInput));
        assert_eq!(renderer.write_str("hello"), Err(ExpectedMoreInput));
        assert_eq!(renderer.move_cursor_to(1, 2), Err(ExpectedMoreInput));
        assert_eq!(renderer.get_cursor_position(), Err(ExpectedMoreInput));
        assert_eq!(
            renderer.set_color(Color::WhiteBright, Color::Red),
            Err(ExpectedMoreInput)
        );
        assert_eq!(renderer.sleep_ms(Milliseconds(0)), Err(ExpectedMoreInput));
        assert_eq!(renderer.getch(), Err(ExpectedMoreInput));
    }

    #[test]
    fn test_empty_input() {
        let input: [Input; 0] = [];
        let mut renderer = RecordedInputRenderer::new(input.iter().cloned());

        assert_eq!(renderer.clear_screen(), Err(ExpectedMoreInput));
        assert_eq!(renderer.flush(), Err(ExpectedMoreInput));
        assert_eq!(renderer.write_str("hello"), Err(ExpectedMoreInput));
        assert_eq!(renderer.move_cursor_to(1, 2), Err(ExpectedMoreInput));
        assert_eq!(renderer.get_cursor_position(), Err(ExpectedMoreInput));
        assert_eq!(
            renderer.set_color(Color::WhiteBright, Color::Red),
            Err(ExpectedMoreInput)
        );
        assert_eq!(renderer.sleep_ms(Milliseconds(0)), Err(ExpectedMoreInput));
        assert_eq!(renderer.getch(), Err(ExpectedMoreInput));
    }
}
