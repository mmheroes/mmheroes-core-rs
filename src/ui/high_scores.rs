use crate::logic::Money;
use crate::ui::cp866_encoding;
use crate::util::TinyString;
use core::convert::TryInto;

pub type HighScore = (tiny_string_ty![128], Money);

pub const SCORE_COUNT: usize = 5;
pub const RECORD_SIZE: usize = 35;
pub const BUFFER_SIZE: usize = SCORE_COUNT * RECORD_SIZE;

pub const MAX_NAME_LENGTH: usize = 32;

pub(crate) fn default_high_scores() -> [HighScore; SCORE_COUNT] {
    [
        (TinyString::from("Коля"), Money(400)),
        (TinyString::from("Саша"), Money(280)),
        (TinyString::from("Эндрю"), Money(180)),
        (TinyString::from("Паша"), Money(100)),
        (TinyString::from("Гриша"), Money(20)),
    ]
}

pub fn decode(mut buffer: &[u8]) -> Option<[HighScore; SCORE_COUNT]> {
    use core::cmp::min;

    if buffer.len() < BUFFER_SIZE {
        return None;
    }

    let mut loaded = default_high_scores();
    for i in 0..SCORE_COUNT {
        let name_length = min(buffer[0] as usize, MAX_NAME_LENGTH);
        buffer = &buffer[1..];
        let name = cp866_encoding::string_from_cp866(&buffer[..name_length]);
        let score = i16::from_le_bytes(
            buffer[MAX_NAME_LENGTH..(MAX_NAME_LENGTH + 2)]
                .try_into()
                .unwrap(),
        );
        buffer = &buffer[(MAX_NAME_LENGTH + 2)..];
        loaded[i] = (TinyString::from(&*name), Money(score));
    }
    Some(loaded)
}

pub fn encode(scores: &[HighScore; SCORE_COUNT]) -> [u8; BUFFER_SIZE] {
    let mut result = [0u8; BUFFER_SIZE];
    let mut buffer: &mut [u8] = &mut result;
    for (name, score) in scores.iter() {
        let (length, rest) = buffer.split_first_mut().unwrap();
        buffer = rest;
        *length =
            cp866_encoding::string_to_cp866_lossy(&*name, &mut buffer[..MAX_NAME_LENGTH])
                as u8;
        buffer[MAX_NAME_LENGTH..(MAX_NAME_LENGTH + 2)]
            .clone_from_slice(&score.0.to_le_bytes());
        buffer = &mut buffer[(MAX_NAME_LENGTH + 2)..]
    }
    result
}
