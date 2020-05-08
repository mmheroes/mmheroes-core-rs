use crate::ui::{Color, Milliseconds, WaitingState};

use core::fmt::Write;

#[derive(Debug)]
pub enum RendererRequest<'a> {
    ClearScreen,
    Flush,
    WriteStr(&'a str),
    MoveCursor {
        line: u8,
        column: u8,
    },
    SetColor {
        foreground: Color,
        background: Color,
    },
    Sleep(Milliseconds),
}

/// Структура для компактного хранения `RendererRequest`'ов.
/// Мы должны хранить строки для `WriteStr`, но так как у нас no_std,
/// то мы не можем использовать `String`. Использовать большую inline-строку
/// будет очень накладно, так как на каждый запрос (даже с очень маленькой строкой)
/// будет расходоваться несколько сотен байтов.
///
/// Вместо этого мы сериализуем запросы в бинарный формат. Для каждого запроса
/// мы сначала записываем его тег, а потом его данные.
///
/// Для `WriteStr` мы после тега записываем длину строки, а потом и саму строку.
/// Таким образом всегда допускается однозначная сериализация.
///
/// Следует обратить внимание, что этот формат не учитывает endianness, и вообще
/// очень простой, так как не предназначен для сохранения данных за пределами текущего
/// процесса.
struct RendererRequestQueue {
    encoded: tiny_vec_ty![u8; 2048],
}

impl RendererRequestQueue {
    fn push(&mut self, request: RendererRequest<'_>) {
        match request {
            RendererRequest::ClearScreen => self.encoded.push(b'C'),
            RendererRequest::Flush => self.encoded.push(b'F'),
            RendererRequest::WriteStr(s) => {
                self.encoded.push(b'W');
                self.encoded
                    .extend_from_slice(&(s.len() as u16).to_ne_bytes());
                self.encoded.extend_from_slice(s.as_bytes());
            }
            RendererRequest::MoveCursor { line, column } => {
                self.encoded.push(b'M');
                self.encoded.extend_from_slice(&line.to_ne_bytes());
                self.encoded.extend_from_slice(&column.to_ne_bytes());
            }
            RendererRequest::SetColor {
                foreground,
                background,
            } => {
                self.encoded.push(b'S');
                self.encoded
                    .push(((foreground as u8) << 4) | ((background as u8) & 0xF));
            }
            RendererRequest::Sleep(ms) => {
                self.encoded.push(b's');
                self.encoded.extend_from_slice(&ms.0.to_ne_bytes());
            }
        }
    }

    pub(in crate::ui) fn iter(&self) -> RendererRequestIter<'_> {
        RendererRequestIter {
            encoded: &*self.encoded,
        }
    }

    pub(in crate::ui) fn clear(&mut self) {
        self.encoded.clear()
    }
}

pub struct RendererRequestIter<'q> {
    pub(crate) encoded: &'q [u8],
}

impl<'q> RendererRequestIter<'q> {
    fn consume_n_bytes(&mut self, len: usize) -> &'q [u8] {
        let (result, rest) = self.encoded.split_at(len);
        self.encoded = rest;
        result
    }

    fn consume_byte(&mut self) -> u8 {
        let byte = self.encoded[0];
        self.encoded = &self.encoded[1..];
        byte
    }
}

impl<'q> Iterator for RendererRequestIter<'q> {
    type Item = RendererRequest<'q>;

    fn next(&mut self) -> Option<Self::Item> {
        use core::convert::TryInto;
        use core::mem::size_of;

        if self.encoded.is_empty() {
            return None;
        }

        let tag = self.consume_byte();

        let request = match tag {
            b'C' => RendererRequest::ClearScreen,
            b'F' => RendererRequest::Flush,
            b'W' => {
                let len_bytes = self.consume_n_bytes(size_of::<u16>());
                let len = u16::from_ne_bytes(len_bytes.try_into().unwrap()) as usize;
                let str_bytes = self.consume_n_bytes(len);
                RendererRequest::WriteStr(core::str::from_utf8(str_bytes).unwrap())
            }
            b'M' => {
                let line_bytes = self.consume_n_bytes(size_of::<Line>());
                let column_bytes = self.consume_n_bytes(size_of::<Column>());
                RendererRequest::MoveCursor {
                    line: Line::from_ne_bytes(line_bytes.try_into().unwrap()),
                    column: Column::from_ne_bytes(column_bytes.try_into().unwrap()),
                }
            }
            b'S' => {
                let color = self.consume_byte();
                let foreground = color >> 4;
                let background = color & 0xF;
                RendererRequest::SetColor {
                    foreground: foreground.try_into().unwrap(),
                    background: background.try_into().unwrap(),
                }
            }
            b's' => {
                let ms_bytes = self.consume_n_bytes(size_of::<i32>());
                RendererRequest::Sleep(Milliseconds(i32::from_ne_bytes(
                    ms_bytes.try_into().unwrap(),
                )))
            }
            _ => panic!("Unknown tag"),
        };
        Some(request)
    }
}

pub type Line = u8;

pub type Column = u8;

pub(in crate::ui) struct Renderer {
    request_queue: RendererRequestQueue,
    line: Line,
    column: Column,
    pub(in crate::ui) waiting_state: Option<WaitingState>,
}

impl Renderer {
    pub(in crate::ui) fn new() -> Renderer {
        Renderer {
            request_queue: RendererRequestQueue {
                encoded: <tiny_vec_ty![u8; 2048]>::new(),
            },
            line: 0,
            column: 0,
            waiting_state: None,
        }
    }

    pub(in crate::ui) fn requests(&self) -> RendererRequestIter {
        self.request_queue.iter()
    }

    pub(in crate::ui) fn clear(&mut self) {
        self.request_queue.clear()
    }

    pub(in crate::ui) fn clear_screen(&mut self) {
        self.column = 0;
        self.line = 0;
        self.request_queue.push(RendererRequest::ClearScreen)
    }

    pub(in crate::ui) fn flush(&mut self) {
        self.request_queue.push(RendererRequest::Flush)
    }

    pub(in crate::ui) fn write_str(&mut self, s: &str) {
        use unicode_segmentation::UnicodeSegmentation;

        if s.is_empty() {
            return;
        }

        for grapheme in s.graphemes(true) {
            if grapheme == "\n" {
                self.column = 0;
                self.line += 1;
            } else {
                self.column += 1;
            }
        }
        self.request_queue.push(RendererRequest::WriteStr(s));
    }

    pub(in crate::ui) fn move_cursor_to(&mut self, line: Line, column: Column) {
        self.line = line;
        self.column = column;
        self.request_queue
            .push(RendererRequest::MoveCursor { line, column })
    }

    pub(in crate::ui) fn get_cursor_position(&mut self) -> (Line, Column) {
        (self.line, self.column)
    }

    pub(in crate::ui) fn set_color(&mut self, foreground: Color, background: Color) {
        self.request_queue.push(RendererRequest::SetColor {
            foreground,
            background,
        })
    }

    pub(in crate::ui) fn sleep_ms(&mut self, ms: Milliseconds) {
        self.request_queue.push(RendererRequest::Sleep(ms))
    }

    pub(in crate::ui) fn write_fmt(&mut self, fmt: core::fmt::Arguments<'_>) {
        struct Adapter<'r>(&'r mut Renderer);

        impl core::fmt::Write for Adapter<'_> {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.0.write_str(s);
                Ok(())
            }
        }

        let mut adapter = Adapter(self);
        adapter.write_fmt(fmt).expect("Invalid format")
    }
}
