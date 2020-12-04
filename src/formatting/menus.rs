//TODO : traduire EN, documenter tout, deplacer struct
/**
 * Crée un menu simple prédéfini avec options numérotées et alignées à gauche (possibilité de les centrer avec le paramètre correspondant).
 * Checker que la valeur n'est pas None avant utilisation.
 * Donner dans le Vector au moins 1 &str de taille supérieure à 2.
 * Possibilité de donner un titre avec une Option<&str>, le titre sera centré.
 */
pub fn default_ordered_menu(
    parts: Vec<&str>,
    title: Option<&str>,
    centered: bool,
) -> Option<String> {
    if parts.len() == 0 {
        None
    } else {
        let mut max_len = 0;
        for i in &parts {
            if i.chars().count() > max_len {
                max_len = i.chars().count();
            }
        }
        if max_len <= 2 {
            None
        } else {
            let nbr_digits = parts.len().to_string().chars().count();
            let mut title_present = false;
            let mut title_disp = "";
            match &title {
                Some(x) => {
                    title_present = true;
                    title_disp = *x;
                    if max_len + nbr_digits - 2 < title_disp.chars().count() {
                        max_len = title_disp.chars().count();
                    }
                }
                None => (),
            }

            let mut ret_val = '+'.to_string();
            ret_val.push_str(&format!("{}\n", "-".repeat(&max_len + nbr_digits + 4),));
            let bot_line = &ret_val.clone();
            let free_line = &format!("|{}|\n", " ".repeat(max_len + nbr_digits + 4));
            ret_val.push_str(free_line);

            if title_present {
                let space_nbr: usize = bot_line.chars().count() - 3 - title_disp.chars().count();
                if space_nbr % 2 == 0 {
                    ret_val.push_str(&format!(
                        "|{}{}{}|\n{}",
                        " ".repeat(space_nbr / 2),
                        title_disp,
                        " ".repeat(space_nbr / 2),
                        free_line
                    ));
                } else {
                    ret_val.push_str(&format!(
                        "|{}{}{}|\n{}",
                        " ".repeat((space_nbr as f32 / 2.0).ceil() as usize),
                        title_disp,
                        " ".repeat((space_nbr as f32 / 2.0).floor() as usize),
                        free_line
                    ));
                }
            }

            let mut order_count = 0;
            for i in &parts {
                order_count += 1;
                if centered {
                    let space_nbr = max_len + 2 - i.chars().count();
                    if space_nbr % 2 == 0 {
                        ret_val.push_str(&format!(
                            "|{}{}. {}{}|\n",
                            " ".repeat(space_nbr / 2),
                            order_count,
                            i,
                            " ".repeat(space_nbr / 2),
                        ));
                    } else {
                        ret_val.push_str(&format!(
                            "|{}{}. {}{}|\n",
                            " ".repeat((space_nbr as f32 / 2.0).ceil() as usize),
                            order_count,
                            i,
                            " ".repeat((space_nbr as f32 / 2.0).floor() as usize),
                        ));
                    }
                } else {
                    let mut pushing_val = format!("| {}. {}", order_count, i);
                    let space_nbr = max_len + nbr_digits + 5 - pushing_val.chars().count();
                    pushing_val.push_str(&" ".repeat(space_nbr));
                    pushing_val.push_str("|\n");
                    ret_val.push_str(&pushing_val);
                }
            }
            ret_val.push_str(free_line);
            ret_val.push_str(bot_line);

            Some(ret_val)
        }
    }
}

pub struct Frame {
    pub angle: char,
    pub horizontal: char,
    pub vertical: char,
}

impl Frame {
    pub fn make_frame(angle: char, horizontal: char, vertical: char) -> Frame {
        Frame {
            angle,
            horizontal,
            vertical,
        }
    }
}

pub fn perso_ordered_menu(
    parts: Vec<&str>,
    title: Option<&str>,
    frame: Frame,
    centered: bool,
) -> Option<String> {
    if parts.len() == 0 {
        None
    } else {
        let mut max_len = 0;
        for i in &parts {
            if i.chars().count() > max_len {
                max_len = i.chars().count();
            }
        }
        if max_len <= 2 {
            None
        } else {
            let angle = frame.angle.to_string();
            let horizontal = frame.horizontal.to_string();
            let vertical = frame.vertical.to_string();

            let nbr_digits = parts.len().to_string().chars().count();
            let mut title_present = false;
            let mut title_disp = "";
            match &title {
                Some(x) => {
                    title_present = true;
                    title_disp = *x;
                    if max_len + nbr_digits - 2 < title_disp.chars().count() {
                        max_len = title_disp.chars().count();
                    }
                }
                None => (),
            }

            let mut ret_val = angle.clone();
            ret_val.push_str(&format!(
                "{}{}\n",
                &horizontal.repeat(&max_len + nbr_digits + 4),
                &angle,
            ));
            let bot_line = &ret_val.clone();
            let free_line = &format!(
                "{}{}{}\n",
                &vertical,
                " ".repeat(max_len + nbr_digits + 4),
                &vertical
            );
            ret_val.push_str(free_line);

            if title_present {
                let space_nbr: usize = bot_line.chars().count() - 3 - title_disp.chars().count();
                if space_nbr % 2 == 0 {
                    ret_val.push_str(&format!(
                        "{}{}{}{}{}\n{}",
                        &vertical,
                        " ".repeat(space_nbr / 2),
                        title_disp,
                        " ".repeat(space_nbr / 2),
                        &vertical,
                        free_line
                    ));
                } else {
                    ret_val.push_str(&format!(
                        "{}{}{}{}{}\n{}",
                        &vertical,
                        " ".repeat((space_nbr as f32 / 2.0).ceil() as usize),
                        title_disp,
                        " ".repeat((space_nbr as f32 / 2.0).floor() as usize),
                        &vertical,
                        free_line
                    ));
                }
            }

            let mut order_count = 0;
            for i in &parts {
                order_count += 1;
                if centered {
                    let space_nbr = max_len + 2 - i.chars().count();
                    if space_nbr % 2 == 0 {
                        ret_val.push_str(&format!(
                            "{}{}{}. {}{}{}\n",
                            &vertical,
                            " ".repeat(space_nbr / 2),
                            order_count,
                            i,
                            " ".repeat(space_nbr / 2),
                            &vertical,
                        ));
                    } else {
                        ret_val.push_str(&format!(
                            "{}{}{}. {}{}{}\n",
                            &vertical,
                            " ".repeat((space_nbr as f32 / 2.0).ceil() as usize),
                            order_count,
                            i,
                            " ".repeat((space_nbr as f32 / 2.0).floor() as usize),
                            &vertical,
                        ));
                    }
                } else {
                    let mut pushing_val = format!("{} {}. {}", &vertical, order_count, i);
                    let space_nbr = max_len + nbr_digits + 5 - pushing_val.chars().count();
                    pushing_val.push_str(&" ".repeat(space_nbr));
                    pushing_val.push_str(&format!("{}\n", &vertical));
                    ret_val.push_str(&pushing_val);
                }
            }
            ret_val.push_str(free_line);
            ret_val.push_str(bot_line);

            Some(ret_val)
        }
    }
}
