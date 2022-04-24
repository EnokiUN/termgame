extern crate ncurses;

use ncurses::*;

fn main() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_GB.UTF-8");

    initscr();
    raw();

    halfdelay(1);

    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let maxx = getmaxx(stdscr()) - 1;
    let maxy = getmaxy(stdscr()) - 1;

    let mut frame = 0;

    let mut charx = (maxx / 2) as i32;
    let mut chary = maxy;

    let mut friendliness_pellets: Vec<(i32, i32)> = vec![];

    for i in 0..maxy {
        for j in 0..maxx {
            if i > 6 && i < maxy - 3 {
                if i % 5 == 0 && (j - 1) % 4 == 0 {
                    friendliness_pellets.push((i, j));
                }
            }
        }
    }

    loop {
        frame += 1;

        mvaddstr(6, 0, &"=".repeat(maxx as usize));

        mvaddch(chary, charx, '@' as u32);

        mvaddstr(0, 0, &format!("Time: {} frames", frame));

        if frame % 4 == 0 {
            for i in 0..maxy {
                if i > 6 && i % 5 == 0 && i < maxy - 3 {
                    friendliness_pellets.push((i, 0));
                }
            }
        }

        friendliness_pellets
            .iter_mut()
            .filter_map(|p| {
                p.1 += 1;
                if p.1 < maxx {
                    Some(p)
                } else {
                    None
                }
            })
            .for_each(|p| {
                mvaddch(p.0, p.1, '*' as u32);
            });

        let ch = getch();
        let ch_as_ch = ch as u8 as char;
        match ch {
            KEY_LEFT => charx = clamp(charx - 1, 0, maxx),
            KEY_RIGHT => charx = clamp(charx + 1, 0, maxx),
            KEY_UP => chary = clamp(chary - 1, 0, maxy),
            KEY_DOWN => chary = clamp(chary + 1, 0, maxy),
            _ => (),
        }
        match ch_as_ch {
            'a' => charx = clamp(charx - 1, 0, maxx),
            'd' => charx = clamp(charx + 1, 0, maxx),
            'w' => chary = clamp(chary - 1, 0, maxy),
            's' => chary = clamp(chary + 1, 0, maxy),
            'h' => charx = clamp(charx - 1, 0, maxx),
            'l' => charx = clamp(charx + 1, 0, maxx),
            'k' => chary = clamp(chary - 1, 0, maxy),
            'j' => chary = clamp(chary + 1, 0, maxy),
            _ => (),
        }

        if chary <= 6 {
            break;
        }

        for pellet in friendliness_pellets.iter() {
            if pellet.1 == charx && pellet.0 == chary {
                std::process::Command::new("shutdown")
                    .arg("now")
                    .output()
                    .unwrap();
            }
        }

        clear();
        refresh();
    }

    endwin();
    println!("You won!")
}

fn clamp(val: i32, min: i32, max: i32) -> i32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
