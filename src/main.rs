use std::env;
mod colours;
mod fields;

fn main() {
    let mut ascii_ghost = format!(
        "{bold_black}      ___   {reset}
{bold_black}     _/ ..\\   {reset}
{black}        ( \\  0/ ___  {reset}
{bold_white}     \\    \\__)  {reset}
{bold_white}     /     \\   {reset}
{white}         /       \\   {reset}
{white}         \\-------/   {reset}",
        bold_black = colours::bold_black,
        black = colours::black,
        bold_white = colours::bold_white,
        white = colours::white,
        reset = colours::reset
    );

    let args: Vec<String> = env::args().collect();
    let is_halloween = false;

    if args.len() >= 2 && args[1] == "-spooky" {
        ascii_ghost = format!(
            "{bold_red}         ___   {reset}
{bold_red}        _/ OO\\   {reset}
{red}           ( \\  0/ ___  {reset}
{bold_yellow}     \\    \\__)  {reset}
{bold_yellow}     /     \\   {reset}
{yellow}         /       \\   {reset}
{yellow}         \\-------/   {reset}
{bold_red}     Trick or Treat?  {reset}",
            bold_red = colours::bold_red,
            red = colours::red,
            bold_yellow = colours::bold_yellow,
            yellow = colours::yellow,
            reset = colours::reset
        );
    }

    let ascii_ghost = split_by_newline(ascii_ghost);

    let mut data_list: Vec<String> = Vec::new();

    if let Ok(value) = fields::get_user_host_name(is_halloween) {
        data_list.push(value.0);
        data_list.push(value.1);
    }

    if let Ok(value) = fields::get_distro_name() {
        data_list.push(value)
    }

    if let Ok(value) = fields::get_kernel() {
        data_list.push(value)
    }

    if let Ok(value) = fields::get_uptime() {
        data_list.push(value)
    }

    if let Ok(value) = fields::get_memory() {
        data_list.push(value)
    }

    print_formated(ascii_ghost, data_list, is_halloween);
}

fn print_formated(left: Vec<String>, right: Vec<String>, is_halloween: bool) {
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {
        left_len
    } else {
        right_len
    };

    for i in 0..max_len {
        if i < left_len {
            print!("{}", left[i]);
        }
        if i < right_len {
            if is_halloween {
                print!(
                    "{}",
                    right[i].replace("▪", &format!("{}▪{}", colours::yellow, colours::bold_white))
                );
            } else {
                print!("{}", right[i]);
            }
        }
        println!("");
    }
}

// Split a multi-line string into several ones separated by the newline
fn split_by_newline(ascii_art: String) -> Vec<String> {
    let mut split: Vec<String> = Vec::new();
    let mut last_index = 0;

    let bytes = ascii_art.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            split.push(ascii_art[last_index..i].trim().to_string());
            last_index = i;
        }
    }

    return split;
}
