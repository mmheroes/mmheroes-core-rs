use crate::ui::{high_scores::HighScore, renderer::Renderer, *};

pub(in crate::ui) fn display_high_scores(
    r: &mut Renderer,
    high_scores: &[HighScore],
) -> WaitingState {
    r.clear_screen();
    r.set_color(Color::WhiteBright, Color::Black);
    writeln!(r, "******                                           ******");
    writeln!(r, "      *********                         *********");
    writeln!(r, "               *************************");
    r.set_color(Color::YellowBright, Color::Black);
    writeln!(r, "Вот имена тех, кто прошел это наводящее ужас испытание:");
    writeln!(r);
    writeln!(r, "    ГЕРОЙ            ЗАРАБОТАЛ");
    r.set_color(Color::WhiteBright, Color::Black);
    for (i, (name, score)) in high_scores.iter().enumerate() {
        r.move_cursor_to((i + 6) as u8, 3);
        write!(r, "{}", name);
        r.move_cursor_to((i + 6) as u8, 24);
        write!(r, "{} руб.", score);
    }
    wait_for_any_key(r)
}
