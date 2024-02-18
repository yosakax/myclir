use std::string;

#[allow(unused)]
use crate::app::{App, AppResult, Mode};
use crossterm;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use itertools::Itertools;
use sqlx::encode::IsNull;
use sqlx::types::Decimal;
use sqlx::{Column, Executor, Row, TypeInfo};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.code == KeyCode::Char('q') && key_event.modifiers == KeyModifiers::CONTROL {
        app.quit();
    }
    match key_event.code {
        KeyCode::F(1) => {
            app.window_number = 0;
            app.is_popup = false;
        }
        KeyCode::F(2) => {
            app.window_number = 1;
        }
        KeyCode::F(3) => {
            app.window_number = 2;
            app.is_popup = false;
        }
        _ => {}
    }
    match app.window_number {
        0 => match key_event.code {
            KeyCode::Up => {
                eprintln!("HERE");
                app.databases.previous();
            }
            KeyCode::Down => {
                app.databases.next();
            }
            _ => {}
        },
        1 => {
            match app.mode {
                Mode::Insert => {
                    // app.is_popup = false;
                    match (key_event.code, key_event.modifiers) {
                        // Exit application on `ESC` or `q`
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            app.is_popup = false;
                            app.text.clear();
                            app.text.push(String::new());
                            app.cursor.initialize();
                            app.line_number = 0;
                        }
                        (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                            app.is_popup = true;
                            app.completion.update();
                        }
                        (KeyCode::Char('['), KeyModifiers::CONTROL) => {
                            app.is_popup = false;
                            app.mode = Mode::Normal;
                        }
                        (KeyCode::Esc, KeyModifiers::NONE) => {
                            if app.is_popup {
                                app.is_popup = false;
                            } else {
                                app.mode = Mode::Normal;
                            }
                        }
                        (KeyCode::Tab, KeyModifiers::NONE) => {
                            for _ in 0..2 {
                                app.text[app.line_number]
                                    .insert((app.cursor.x - app.cursor.base_x) as usize, ' ');
                            }
                            app.cursor.x += 2;
                        }
                        (KeyCode::Backspace, KeyModifiers::NONE) => {
                            app.is_popup = false;
                            if app.cursor.x == app.cursor.base_x {
                                if app.cursor.y != app.cursor.base_y {
                                    app.line_number -= 1;
                                    app.cursor.y -= 1;
                                    app.cursor.x =
                                        app.cursor.base_x + app.text[app.line_number].len() as u16;
                                    let rest_text = app.text[app.line_number + 1].clone();
                                    app.text[app.line_number].push_str(rest_text.as_str());
                                    app.text.remove(app.line_number + 1);
                                }
                            } else {
                                app.cursor.x -= 1;
                                app.text[app.line_number]
                                    .remove((app.cursor.x - app.cursor.base_x) as usize);
                            }
                        }
                        (KeyCode::Left, KeyModifiers::NONE) => {
                            if app.cursor.x > app.cursor.base_x {
                                app.cursor.x -= 1;
                            }
                        }
                        (KeyCode::Right, KeyModifiers::NONE) => {
                            if app.cursor.x - app.cursor.base_x
                                < app.text[app.line_number].len() as u16
                            {
                                app.cursor.x += 1;
                            }
                        }
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if app.cursor.y > app.cursor.base_y {
                                app.cursor.y -= 1;
                                app.line_number -= 1;
                                app.cursor.x = app.cursor.x.min(
                                    app.cursor.base_x + app.text[app.line_number].len() as u16,
                                );
                            }
                        }
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if app.cursor.y - app.cursor.base_y < app.text.len() as u16 - 1 {
                                app.cursor.y += 1;
                                app.line_number += 1;
                                app.cursor.x = app.cursor.x.min(
                                    app.cursor.base_x + app.text[app.line_number].len() as u16,
                                );
                            }
                        }
                        (KeyCode::F(5), _) => {
                            app.result = vec![vec!["Runnning".to_string()]];
                            if app.text[0].is_empty() && app.text.len() == 1 {
                                app.result = vec![vec!["No query input.".to_string()]];
                            } else {
                                let queries = app.text.join(" ").trim().to_string();
                                // run_query();
                                let queries: Vec<&str> = queries.split(';').collect();
                                let mut result = vec![];
                                for &query in queries.iter() {
                                    if !query.is_empty() {
                                        let ret = app.conn.fetch_all(query).await;
                                        match ret {
                                            Ok(ret) => {
                                                let colmun_names = ret[0]
                                                    .columns()
                                                    .iter()
                                                    .map(|x| x.name())
                                                    .collect_vec();

                                                result.push(
                                                    colmun_names
                                                        .iter()
                                                        .map(|x| x.to_string())
                                                        .collect_vec(),
                                                );
                                                let column_types = ret[0]
                                                    .columns()
                                                    .iter()
                                                    .map(|x| x.type_info().name().to_string())
                                                    .collect_vec();

                                                for row in ret.iter() {
                                                    let mut tmp = vec![];
                                                    for i in 0..column_types.len() {
                                                        let col_name = colmun_names[i];
                                                        let col_type = column_types[i].clone();
                                                        match col_type.as_str() {
                                                            "INT" | "BIGINT" => {
                                                                match row
                                                                    .try_get::<i64, &str>(col_name)
                                                                    .is_ok()
                                                                {
                                                                    true => {
                                                                        let val: i64 =
                                                                            row.get(col_name);
                                                                        tmp.push(val.to_string());
                                                                    }
                                                                    false => {
                                                                        tmp.push(
                                                                            "NULL".to_string(),
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            "DECIMAL" => {
                                                                match row
                                                                    .try_get::<Decimal, &str>(
                                                                        col_name,
                                                                    )
                                                                    .is_ok()
                                                                {
                                                                    true => {
                                                                        let val: Decimal =
                                                                            row.get(col_name);
                                                                        tmp.push(val.to_string());
                                                                    }
                                                                    false => {
                                                                        tmp.push(
                                                                            "NULL".to_string(),
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                match row
                                                                    .try_get::<String, &str>(
                                                                        col_name,
                                                                    )
                                                                    .is_ok()
                                                                {
                                                                    true => {
                                                                        let val: String =
                                                                            row.get(col_name);
                                                                        tmp.push(val.to_string());
                                                                    }
                                                                    false => {
                                                                        tmp.push(
                                                                            "NULL".to_string(),
                                                                        );
                                                                    }
                                                                }
                                                                // let val = row.get(col_name);
                                                                // tmp.push(val);
                                                            }
                                                        }
                                                    }
                                                    result.push(tmp);
                                                }
                                                result.push(vec![]);
                                            }
                                            Err(e) => {
                                                result = vec![vec![e.to_string()]];
                                                break;
                                            }
                                        }
                                    }
                                }
                                app.result = result.clone();
                            }
                        }
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            app.is_popup = false;
                            if app.cursor.y - app.cursor.base_y + 3 < app.edit_area.1 {
                                let now_line = app.text[app.line_number].clone();
                                let (left_text, right_text) = now_line
                                    .split_at((app.cursor.x - app.cursor.base_x) as usize)
                                    .to_owned();
                                app.text.remove(app.line_number);
                                app.text.insert(app.line_number, right_text.to_string());
                                app.text.insert(app.line_number, left_text.to_string());
                                app.text.push(String::new());
                                app.line_number += 1;
                                app.cursor.y += 1;
                                app.cursor.x = app.cursor.base_x;
                            }
                        }
                        (KeyCode::Char(char_input), _) => {
                            let idx = (app.cursor.x - app.cursor.base_x) as usize;
                            app.text[app.line_number].insert(idx, char_input);
                            app.cursor.x += 1;
                            app.completion.update();
                            if char_input == ';' || char_input == ' ' {
                                app.is_popup = false;
                            } else {
                                app.is_popup = true;
                            }
                        }
                        _ => {}
                    }
                }
                Mode::Normal => match (key_event.code, app.popup_command) {
                    (KeyCode::Char('i'), false) => {
                        app.mode = Mode::Insert;
                    }
                    (KeyCode::Char(':'), false) => {
                        app.popup_command = true;
                    }
                    (KeyCode::Esc, _) => {
                        app.popup_command = false;
                        app.is_popup = false;
                    }
                    (KeyCode::Char('o'), false) => {
                        app.mode = Mode::Insert;
                        app.text.push(String::new());
                        app.cursor.x = app.cursor.base_x;
                        app.cursor.y += 1;
                        app.line_number += 1;
                    }
                    _ => {}
                },
                Mode::Command => match key_event.code {
                    KeyCode::Esc => {
                        app.command_input.clear();
                        app.popup_command = false;
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Char(char_input) => app.command_input.push(char_input),
                    _ => {}
                },
            }
        }
        2 => match key_event.code {
            KeyCode::Up => {
                app.table_components.previous();
            }
            KeyCode::Down => {
                app.table_components.next();
            }
            _ => {}
        },

        _ => {}
    }
    Ok(())
}
