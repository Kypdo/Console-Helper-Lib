use crate::formatting::menus::Frame;
//peut etre un .rs avec utilities, et le mettre en private (protected ?), ou alors pub use ? à voir

//TODO: Ajouter une option centered:bool
pub fn print_results(frame: Frame, result: &str, result_of: &str, title: Option<&str>) -> String {
    const CURRENT_SENTENCE1: &str = "The result of "; // + result_of
    const CURRENT_SENTENCE2: &str = " is ";
    let mut max_len = CURRENT_SENTENCE1.chars().count()
        + CURRENT_SENTENCE2.chars().count()
        + result.chars().count()
        + result_of.chars().count(); //Current sentences + results length + '.'
    let angle = frame.angle.to_string();
    let horizontal = frame.horizontal.to_string();
    let vertical = frame.vertical.to_string();

    let mut title_present = false;
    let mut title_disp = "";
    match title {
        Some(x) => {
            title_present = true;
            title_disp = x;
            if max_len < title_disp.chars().count() {
                max_len = title_disp.chars().count();
            }
        }
        None => (),
    }
    let mut printing_result = angle.clone();
    printing_result.push_str(&format!(
        "{}{}\n",
        horizontal.repeat(max_len + 2 + 1),
        angle
    )); //repeat : length + both spaces + '.'
    let free_line = &format!(
        "{}{}{}\n",
        vertical,
        " ".repeat(max_len + 2 + 1), // repeat : length + both spaces + '.'
        vertical
    );

    let bot_line = &printing_result.clone();
    printing_result.push_str(free_line);
    if title_present {
        // TODO: DOCUMENTER CE CODE
        let space_nbr: usize = bot_line.chars().count() - 3 - title_disp.chars().count();
        if space_nbr % 2 == 0 {
            printing_result.push_str(&format!(
                "{}{}{}{}{}\n{}",
                &vertical,
                " ".repeat(space_nbr / 2),
                title_disp,
                " ".repeat(space_nbr / 2),
                &vertical,
                free_line
            ));
        } else {
            printing_result.push_str(&format!(
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
    printing_result.push_str(&format!(
        "{} {}{}{}{}. {}\n",
        vertical, CURRENT_SENTENCE1, result_of, CURRENT_SENTENCE2, result, vertical
    )); //TODO! : Bug quand title.length() > max_len, devoir faire ca de facon dyna (pas juste un espace tavu, faire calcul)
    printing_result.push_str(bot_line);

    printing_result
}
