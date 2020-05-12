use crate::logic::Action;
use crate::ui::{renderer::Renderer, *};

pub(in crate::ui) fn display_what_to_do(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    write_colored!(White, r, "Есть всего ");
    write_colored!(YellowBright, r, "6 дней");
    write_colored!(White, r, ". За это время надо успеть получить ");
    write_colored!(YellowBright, r, "6 зачетов");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Чтобы получить ");
    write_colored!(YellowBright, r, "зачет");
    write_colored!(White, r, ", можно успешно сдать сколько-то ");
    write_colored!(YellowBright, r, "заданий");
    writeln_colored!(White, r, ".");

    write_colored!(
        White,
        r,
        "Чтобы сдать несколько заданий, можно чего-то знать и "
    );
    write_colored!(YellowBright, r, "прийти к преподу");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Чтобы чего-то знать, можно ");
    write_colored!(YellowBright, r, "готовиться");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Преподавателей надо искать по ");
    write_colored!(YellowBright, r, "расписанию");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Пока готовишься или сдаешь, ");
    write_colored!(YellowBright, r, "самочуствие");
    writeln_colored!(White, r, " ухудшается.");

    write_colored!(White, r, "Чтобы улучшить самочуствие, можно ");
    write_colored!(YellowBright, r, "отдыхать");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Всякие ");
    write_colored!(YellowBright, r, "дополнительные персонажи");
    writeln_colored!(White, r, " могут помогать, а могут мешать.");

    write_colored!(
        RedBright,
        r,
        "Альтернативные варианты есть почти везде, но они тоже чего-то стоят"
    );
    writeln_colored!(White, r, ".");

    help_dialog(r, available_actions)
}

pub(in crate::ui) fn display_about_screen(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    write_colored!(White, r, "В левом верхнем углу - игровые ");
    write_colored!(YellowBright, r, "дата");
    write_colored!(White, r, " и ");
    write_colored!(YellowBright, r, "время");
    writeln_colored!(White, r, ",");

    write_colored!(White, r, "твое состояние (");
    write_colored!(YellowBright, r, "здоровье");
    write_colored!(White, r, ", ");
    write_colored!(YellowBright, r, "качества");
    write_colored!(White, r, "), ");
    write_colored!(YellowBright, r, "деньги");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "В правом верхнем углу - твои ");
    write_colored!(YellowBright, r, "навыки");
    writeln_colored!(White, r, " по предметам.");

    write_colored!(White, r, "Навыки оцениваются двояко: по ");
    write_colored!(YellowBright, r, "\"общей шкале\"");
    writeln_colored!(White, r, " (число)");

    write_colored!(White, r, "и по ");
    write_colored!(
        YellowBright,
        r,
        "шкале требований конкретного преподавателя"
    );
    writeln_colored!(White, r, " (\"оценка\").");

    writeln_colored!(
        White,
        r,
        "Ниже навыков - мини-расписание на этот день + сданные задачи."
    );

    writeln_colored!(
        White,
        r,
        "Полное расписание можно посмотреть в общаге (выбрать в меню)."
    );

    writeln_colored!(
        White,
        r,
        "Наконец, слева в нижней половине экрана - текущее меню."
    );

    writeln!(r);

    write_colored!(Green, r, " СОСТОЯНИЕ     ");
    writeln_colored!(WhiteBright, r, "НАВЫКИ");
    writeln_colored!(YellowBright, r, " СИТУАЦИЯ");
    write_colored!(CyanBright, r, " МЕНЮ          ");
    writeln_colored!(RedBright, r, "РАСПИСАНИЕ");

    help_dialog(r, available_actions)
}

pub(in crate::ui) fn display_where_to_go_and_why(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    write_colored!(White, r, "В ");
    write_colored!(YellowBright, r, "общаге");
    writeln_colored!(White, r, " ты готовишься и отдыхаешь.");

    write_colored!(White, r, "На ");
    write_colored!(YellowBright, r, "факультете(~=ПУНК)");
    writeln_colored!(White, r, " ты бегаешь по преподам и ищешь приятелей.");

    write_colored!(White, r, "Чтобы попасть в ");
    write_colored!(YellowBright, r, "компьюетрный класс");
    writeln_colored!(White, r, ", надо прийти на факультет.");

    writeln_colored!(
        White,
        r,
        "В компьютерном классе ты сдаешь зачет по информатике и ищешь друзей."
    );

    write_colored!(YellowBright, r, "Мавзолей");
    writeln_colored!(
        White,
        r,
        " - это такая столовая. Там ты отдыхаешь и ищешь приятелей."
    );

    write_colored!(YellowBright, r, "ПОМИ");
    writeln_colored!(
        White,
        r,
        " - Петербургское Отделение Математического Института РАН."
    );

    writeln_colored!(White, r, "В ПОМИ ты будешь искать преподов и приятелей.");

    write_colored!(White, r, "В ПОМИ надо ехать на электричке, это занимает ");
    write_colored!(YellowBright, r, "1 час");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Если ехать зайцем - то может оказаться, что и ");
    write_colored!(YellowBright, r, "2 часа");
    writeln_colored!(White, r, ".");

    write_colored!(White, r, "Кроме того, ");
    write_colored!(RedBright, r, "поездка отнимает и здоровье тоже");
    writeln_colored!(White, r, ".");

    help_dialog(r, available_actions)
}

pub(in crate::ui) fn display_about_professors(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    write_colored!(YellowBright, r, "Всемирнов М.А., алгебра");
    writeln_colored!(White, r, " - очень серьезный и весьма строгий.");

    write_colored!(YellowBright, r, "Дубцов Е.С., матан");
    writeln_colored!(White, r, " - не очень строгий и с некоторой халявой.");

    write_colored!(YellowBright, r, "Подкорытов С.С., геометрия");
    writeln_colored!(White, r, " - замещает Дуткевича Ю.Г.. Почти без проблем.");

    write_colored!(YellowBright, r, "Климов А.А., информатика");
    writeln_colored!(White, r, " - без проблем, но трудно найти.");

    write_colored!(YellowBright, r, "Влащенко Н.П., English");
    writeln_colored!(White, r, " - без проблем, но с некоторым своеобразием.");

    write_colored!(YellowBright, r, "Альбинский Е.Г., Физ-ра");
    writeln_colored!(White, r, " - без проблем, но от физ-ры сильно устаешь.");

    help_dialog(r, available_actions)
}

pub(in crate::ui) fn display_about_characters(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    write_colored!(YellowBright, r, "Diamond");
    writeln_colored!(
        White,
        r,
        " - автор игры \"Герои Мата и Меха\" (MMHEROES), знает всё о ее \"фичах\"."
    );

    write_colored!(YellowBright, r, "Миша");
    writeln_colored!(
        White,
        r,
        " - когда-то альфа-тестер; понимает в стратегии получения зачетов."
    );

    write_colored!(YellowBright, r, "Серж");
    writeln_colored!(
        White,
        r,
        " - еще один экс-альфа-тестер и просто хороший товарищ."
    );

    write_colored!(YellowBright, r, "Паша");
    writeln_colored!(
        White,
        r,
        " - староста. Самый нужный в конце семестра человек."
    );

    write_colored!(YellowBright, r, "RAI");
    writeln_colored!(
        White,
        r,
        " - простой студент. Не любит, когда кто-то НЕ ХОЧЕТ ему помогать."
    );

    write_colored!(YellowBright, r, "Эндрю");
    writeln_colored!(
        White,
        r,
        " - то же студент. Можно попробовать обратиться к нему за помощью."
    );

    write_colored!(YellowBright, r, "Саша");
    writeln_colored!(
        White,
        r,
        " - еще один студент; подробно и разборчиво конспектирует лекции."
    );

    write_colored!(YellowBright, r, "NiL");
    writeln_colored!(
        White,
        r,
        " - девушка из вольнослушателей. Часто эксплуатирует чужие мозги."
    );

    write_colored!(YellowBright, r, "Коля");
    writeln_colored!(White, r, " - студент, большой любитель алгебры и выпивки.");

    write_colored!(YellowBright, r, "Гриша");
    writeln_colored!(White, r, " - студент-пофигист. Любит пиво и халяву.");

    write_colored!(YellowBright, r, "Кузьменко В.Г.");
    writeln_colored!(
        White,
        r,
        " - преподает информатику у другой половины 19-й группы."
    );

    write_colored!(YellowBright, r, "DJuG");
    writeln_colored!(White, r, " - угадайте, кто ;)");

    help_dialog(r, available_actions)
}

pub(in crate::ui) fn display_about_this_program(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    writeln_colored!(WhiteBright, r, "CrWMM Development Team:");
    writeln!(r);

    write_colored!(YellowBright, r, "Дмитрий Петров (aka Diamond)");
    writeln_colored!(White, r, " - автор идеи, главный программист");

    write_colored!(YellowBright, r, "Константин Буленков");
    writeln_colored!(White, r, " - портирование");

    write_colored!(YellowBright, r, "Ваня Павлик");
    writeln_colored!(White, r, " - тестирование, веб-страничка");

    write_colored!(YellowBright, r, "Алексей Румянцев (aka RAI)");
    writeln_colored!(White, r, " - retired веб-мастер");

    writeln_colored!(
        White,
        r,
        "Мнение авторов не всегда совпадает с высказываниями персонажей."
    );
    writeln!(r);

    write_colored!(CyanBright, r, "Если запустить ");
    write_colored!(WhiteBright, r, "mmheroes");
    writeln_colored!(
        CyanBright,
        r,
        " с хоть каким параметром, у тебя будет возможность"
    );

    writeln_colored!(
        CyanBright,
        r,
        "выбрать личный профиль своего \"героя\"; например,"
    );
    writeln_colored!(Green, r, "           mmheroes z#11");
    writeln_colored!(CyanBright, r, "Появится менюшка, в которой все и так ясно.");

    help_dialog(r, available_actions)
}

fn help_dialog(r: &mut Renderer, available_actions: &[Action]) -> WaitingState {
    r.move_cursor_to(13, 0);
    writeln_colored!(White, r, "Что тебя интересует?");
    dialog(r, dialog_options_for_actions(available_actions))
}
