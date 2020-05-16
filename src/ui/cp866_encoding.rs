//! Таблица символов для кодировки CP866, в которой мы сохраняем файл
//! с рекордами. Почему мы сохраняем файл с рекордами в этой кодировке?
//! Потому что оригинальная игра это делала. Это же круто, когда сохраняется
//! совместимость с оригинальной игрой, правда? :)

const CHARACTER_MAP: [char; 256] = [
    // Первая половина полностью совпадает с ASCII.
    '\x00', '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\x09',
    '\x0A', '\x0B', '\x0C', '\x0D', '\x0E', '\x0F', '\x10', '\x11', '\x12', '\x13',
    '\x14', '\x15', '\x16', '\x17', '\x18', '\x19', '\x1A', '\x1B', '\x1C', '\x1D',
    '\x1E', '\x1F', '\x20', '\x21', '\x22', '\x23', '\x24', '\x25', '\x26', '\x27',
    '\x28', '\x29', '\x2A', '\x2B', '\x2C', '\x2D', '\x2E', '\x2F', '\x30', '\x31',
    '\x32', '\x33', '\x34', '\x35', '\x36', '\x37', '\x38', '\x39', '\x3A', '\x3B',
    '\x3C', '\x3D', '\x3E', '\x3F', '\x40', '\x41', '\x42', '\x43', '\x44', '\x45',
    '\x46', '\x47', '\x48', '\x49', '\x4A', '\x4B', '\x4C', '\x4D', '\x4E', '\x4F',
    '\x50', '\x51', '\x52', '\x53', '\x54', '\x55', '\x56', '\x57', '\x58', '\x59',
    '\x5A', '\x5B', '\x5C', '\x5D', '\x5E', '\x5F', '\x60', '\x61', '\x62', '\x63',
    '\x64', '\x65', '\x66', '\x67', '\x68', '\x69', '\x6A', '\x6B', '\x6C', '\x6D',
    '\x6E', '\x6F', '\x70', '\x71', '\x72', '\x73', '\x74', '\x75', '\x76', '\x77',
    '\x78', '\x79', '\x7A', '\x7B', '\x7C', '\x7D', '\x7E', '\x7F',
    // Дальше идут кириллические и специальные символы
    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
    'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я', 'а', 'б',
    'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', '░', '▒', '▓',
    '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', '└', '┴', '┬', '├',
    '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', '╨', '╤', '╥', '╙', '╘',
    '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', 'р', 'с', 'т', 'у', 'ф', 'х',
    'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я', 'Ё', 'ё', 'Є', 'є', 'Ї', 'ї', 'Ў',
    'ў', '°', '∙', '·', '√', '№', '¤', '■', ' ',
];

pub(crate) const fn char_from_cp866(cp866_encoded: u8) -> char {
    CHARACTER_MAP[cp866_encoded as usize]
}

pub(crate) fn string_from_cp866(cp866_encoded: &[u8]) -> tiny_string_ty![128] {
    cp866_encoded
        .iter()
        .cloned()
        .take_while(|&b| b != 0)
        .map(char_from_cp866)
        .collect()
}

pub(crate) fn char_to_cp866_lossy(c: char) -> u8 {
    let result = match c {
        '\x00'..='\x7F' => c as u32,
        'А'..='п' => (0x80 + (c as u32) - ('А' as u32)),
        'р'..='я' => (0xE0 + (c as u32) - ('р' as u32)),
        '░'..='▓' => (0xB0 + (c as u32) - ('░' as u32)),
        '│' => 0xB3,
        '┤' => 0xB4,
        '╡' => 0xB5,
        '╢' => 0xB6,
        '╖' => 0xB7,
        '╕' => 0xB8,
        '╣' => 0xB9,
        '║' => 0xBA,
        '╗' => 0xBB,
        '╝' => 0xBC,
        '╜' => 0xBD,
        '╛' => 0xBE,
        '┐' => 0xBF,
        '└' => 0xC0,
        '┴' => 0xC1,
        '┬' => 0xC2,
        '├' => 0xC3,
        '─' => 0xC4,
        '┼' => 0xC5,
        '╞' => 0xC6,
        '╟' => 0xC7,
        '╚' => 0xC8,
        '╔' => 0xC9,
        '╩' => 0xCA,
        '╦' => 0xCB,
        '╠' => 0xCC,
        '═' => 0xCD,
        '╬' => 0xCE,
        '╧' => 0xCF,
        '╨' => 0xD0,
        '╤' => 0xD1,
        '╥' => 0xD2,
        '╙' => 0xD3,
        '╘' => 0xD4,
        '╒' => 0xD5,
        '╓' => 0xD6,
        '╫' => 0xD7,
        '╪' => 0xD8,
        '┘' => 0xD9,
        '┌' => 0xDA,
        '█' => 0xDB,
        '▄' => 0xDC,
        '▌' => 0xDD,
        '▐' => 0xDE,
        '▀' => 0xDF,
        'Ё' => 0xF0,
        'ё' => 0xF1,
        'Є' => 0xF2,
        'є' => 0xF3,
        'Ї' => 0xF4,
        'ї' => 0xF5,
        'Ў' => 0xF6,
        'ў' => 0xF7,
        '°' => 0xF8,
        '∙' => 0xF9,
        '·' => 0xFA,
        '√' => 0xFB,
        '№' => 0xFC,
        '¤' => 0xFD,
        '■' => 0xFE,
        _ => b'?' as u32,
    };
    result as u8
}

pub(crate) fn string_to_cp866_lossy(s: &str, output: &mut [u8]) -> usize {
    let mut bytes_written = 0;
    for (input, output) in s.chars().map(char_to_cp866_lossy).zip(output.iter_mut()) {
        bytes_written += 1;
        *output = input
    }
    bytes_written
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(input: &str) -> tiny_string_ty![128] {
        let mut buffer = [0u8; 128];
        string_to_cp866_lossy(input, &mut buffer);
        string_from_cp866(&buffer)
    }

    #[test]
    fn test_from_cp866_to_utf8() {
        assert_eq!(string_from_cp866(&[0x8A, 0xAE, 0xAB, 0xEF]), "Коля");
        assert_eq!(string_from_cp866(&[0x91, 0xA0, 0xE8, 0xA0]), "Саша");
        assert_eq!(string_from_cp866(&[0x9D, 0xAD, 0xA4, 0xE0, 0xEE]), "Эндрю");
        assert_eq!(string_from_cp866(&[0x8F, 0xA0, 0xE8, 0xA0]), "Паша");

        assert_eq!(string_from_cp866(&[b'?', 0, 0, 0]), "?");
    }

    #[test]
    fn test_from_utf8_to_cp866() {
        let mut buffer = [0u8; 5];
        string_to_cp866_lossy("Коля", &mut buffer);
        assert_eq!(buffer, [0x8A, 0xAE, 0xAB, 0xEF, 0x00]);
        buffer = [0u8; 5];
        string_to_cp866_lossy("Саша", &mut buffer);
        assert_eq!(buffer, [0x91, 0xA0, 0xE8, 0xA0, 0x00]);
        buffer = [0u8; 5];
        string_to_cp866_lossy("Эндрю", &mut buffer);
        assert_eq!(buffer, [0x9D, 0xAD, 0xA4, 0xE0, 0xEE]);
        buffer = [0u8; 5];
        string_to_cp866_lossy("Паша", &mut buffer);
        assert_eq!(buffer, [0x8F, 0xA0, 0xE8, 0xA0, 0x00]);
    }

    #[test]
    fn test_round_trip() {
        assert_eq!(round_trip("Лёва"), "Лёва");
        assert_eq!(
            round_trip("Съешь же ещё этих мягких французских булок, да выпей чаю"),
            "Съешь же ещё этих мягких французских булок, да выпей чаю"
        );
        assert_eq!(round_trip("👩‍👧‍👧"), "?????");
        assert_eq!(round_trip("▒█ёў┐┘"), "▒█ёў┐┘");
    }
}
