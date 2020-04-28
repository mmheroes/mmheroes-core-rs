use crate::logic::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C)]
pub enum Color {
    Black = 0,
    White = 7,
    Gray = 8,
    Red = 9,
    Green = 10,
    Yellow = 11,
    Cyan = 14,
    WhiteBright = 15,
}

pub const ALL_COLORS: [Color; 8] = [
    Color::Black,
    Color::White,
    Color::Gray,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Cyan,
    Color::WhiteBright,
];

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Input {
    KeyUp,
    KeyDown,
    Enter,
    Other,
    EOF,
}

pub trait Renderer {
    fn clear_screen(&mut self);
    fn flush(&mut self);
    fn move_cursor_to(&mut self, line: i32, column: i32);
    fn get_cursor_position(&mut self) -> (i32, i32);
    fn set_color(&mut self, foreground: Color, background: Color);
    fn write_str<S: AsRef<str>>(&mut self, string: S);
    fn getch(&mut self) -> Input;
    fn sleep_ms(&mut self, ms: Milliseconds);
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Milliseconds(pub i32);

pub struct GameUI<'r, R: Renderer> {
    renderer: &'r mut R,
    game: Game,
}

impl<'r, R: Renderer> GameUI<'r, R> {
    pub fn new(renderer: &'r mut R, game: Game) -> Self {
        Self { renderer, game }
    }

    pub fn run(&mut self) {
        loop {
            use GameState::*;
            self.renderer.clear_screen();
            let action = match self.game.state() {
                Start => Action::_0,
                Terminal => break,
                Intro => display_intro(self.renderer),
                InitialParameters => display_initial_parameters(self.renderer, self.game.mode()),
                Ding(_) => display_ding(self.renderer),
                GameState::Timetable(player, timetable) => {
                    display_timetable(self.renderer, timetable)
                }
                SceneRouter(player, location) => display_scene_router(self.renderer, *location),
            };
            self.game.perform_action(action);
        }
    }
}

fn sleep(r: &mut impl Renderer, ms: Milliseconds) {
    r.flush();
    r.sleep_ms(ms);
}

fn wait_for_any_key(r: &mut impl Renderer) -> Action {
    r.move_cursor_to(23, 0);
    r.set_color(Color::Yellow, Color::Black);
    r.write_str("Нажми любую клавишу ...");
    r.flush();
    if let Input::EOF = r.getch() {
        Action::Exit
    } else {
        Action::_0
    }
}

fn display_intro(r: &mut impl Renderer) -> Action {
    r.set_color(Color::Gray, Color::Black);
    r.write_str("                                                Нам понятен этот смех\n");
    r.write_str("                                                Не попавших на Мат-Мех\n");
    r.write_str("                                                  (надпись на парте)\n");
    r.write_str("\n\n\n");
    r.set_color(Color::WhiteBright, Color::Black);
    r.write_str(" H H  EEE  RR    O   EEE  SS       M   M  A   A TTTTT       M   M  EEE  X   X\n");
    r.write_str(" H H  E    R R  O O  E   S         MM MM  AAAAA   T         MM MM    E   X X\n");
    r.write_str(" HHH  EE   RR   O O  EE   S    OF  M M M  A   A   T    &&&  M M M   EE    X\n");
    r.write_str(" H H  E    R R  O O  E     S       M   M   A A    T         M   M    E   X X\n");
    r.write_str(" H H  EEE  R R   O   EEE SS        M   M    A     T         M   E  EEE  X   X\n");
    r.write_str("\n\n");
    r.set_color(Color::Red, Color::Black);
    r.write_str("                             ГЕРОИ МАТА И МЕХА ;)\n");
    r.write_str("\n\n");
    r.set_color(Color::Cyan, Color::Black);
    r.write_str("(P) CrWMM Development Team, 2001.\n");
    r.write_str("Версия gamma3.14.\n");
    r.write_str("Загляните на нашу страничку: mmheroes.chat.ru !\n");
    wait_for_any_key(r)
}

fn dialog(r: &mut impl Renderer, options: &[(&str, Color)]) -> Action {
    use std::convert::{TryFrom, TryInto};

    let options_count: i16 = options.len().try_into().expect("Too many options given!");

    let mut current_choice = 0i16;
    let start = r.get_cursor_position();
    loop {
        let mut chosen_line_end_position = start;
        for (i, &(name, color)) in options.iter().enumerate() {
            if i == current_choice as usize {
                r.set_color(Color::Black, Color::White);
            } else {
                r.set_color(color, Color::Black);
            }
            r.write_str(name);
            if i == current_choice as usize {
                chosen_line_end_position = r.get_cursor_position();
            }
            r.write_str("\n");
        }
        r.move_cursor_to(chosen_line_end_position.0, chosen_line_end_position.1);

        match r.getch() {
            Input::KeyDown => {
                current_choice = (options_count + current_choice + 1) % options_count;
            }
            Input::KeyUp => {
                current_choice = (options_count + current_choice - 1) % options_count;
            }
            Input::Enter => {
                return Action::try_from(current_choice).expect("Unexpected action number")
            }
            Input::Other => (),
            Input::EOF => return Action::Exit,
        }
        r.move_cursor_to(start.0, start.1);
    }
}

fn display_initial_parameters(r: &mut impl Renderer, mode: GameMode) -> Action {
    debug_assert!(mode == GameMode::God || mode == GameMode::SelectInitialParameters);
    r.set_color(Color::White, Color::Black);
    r.write_str("Выбери начальные параметры своего \"героя\":\n\n");

    let options = &[
        ("Случайный студент", Color::Cyan),
        ("Шибко умный", Color::Cyan),
        ("Шибко наглый", Color::Cyan),
        ("Шибко общительный", Color::Cyan),
        ("GOD-режим", Color::Cyan),
    ];

    dialog(
        r,
        if mode == GameMode::God {
            options
        } else {
            &options[..(options.len() - 1)]
        },
    )
}

fn display_ding(r: &mut impl Renderer) -> Action {
    r.set_color(Color::Green, Color::Black);
    r.write_str("ДЗИНЬ!\n");
    sleep(r, Milliseconds(500));
    r.set_color(Color::Yellow, Color::Black);
    r.write_str("ДДДЗЗЗЗЗИИИИИИННННННЬ !!!!\n");
    sleep(r, Milliseconds(700));
    r.set_color(Color::Red, Color::Black);
    r.write_str("ДДДДДДЗЗЗЗЗЗЗЗЗЗЗЗЗИИИИИИИИИИННННННННННННЬ !!!!!!!!!!\n");
    sleep(r, Milliseconds(1000));
    r.set_color(Color::White, Color::Black);
    r.write_str("Ты просыпаешься от звонка будильника 22-го мая в 8:00.\n");
    r.write_str("Неожиданно ты осознаешь, что началась зачетная неделя,\n");
    r.write_str("а твоя готовность к этому моменту практически равна нулю.\n");
    r.write_str("Натягивая на себя скромное одеяние студента,\n");
    r.write_str("ты всматриваешься в заботливо оставленное соседом на стене\n");
    r.write_str("расписание: когда и где можно найти искомого препода ?\n");
    wait_for_any_key(r)
}

fn display_timetable(r: &mut impl Renderer, timetable: &Timetable) -> Action {
    todo!()
}

fn display_scene_router(r: &mut impl Renderer, location: Location) -> Action {
    todo!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
