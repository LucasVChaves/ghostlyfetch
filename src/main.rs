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

    if args.len() >= 2 && args[1] == "-spooky"{
        ascii_ghost = format!(
"{bold_red}         ___   {reset}
{bold_red}        _/ OO\\   {reset}
{red}           ( \\  0/ ___  {reset}
{bold_yellow}     \\    \\__)  {reset}
{bold_yellow}     /     \\   {reset}
{yellow}         /       \\   {reset}
{yellow}         \\-------/   {reset}",
        bold_red = colours::bold_red,
        red = colours::red,
        bold_yellow = colours::bold_yellow,
        yellow = colours::yellow,
        reset = colours::reset
        );
    }

    let ascii_ghost = split_by_newline(ascii_ghost);

    let mut data_list: Vec<String> = Vec::new();

    match fields::get_user_host_name(is_halloween) {
        Ok(value) => {
            data_list.push(value.0);
            data_list.push(value.1);
        },
        Err(_) => {}
    };

    match fields::get_distro_name(){
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    match fields::get_kernel(){
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    match fields::get_shell(){
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    match fields::get_uptime() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    match fields::get_memory() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    print_formated(ascii_ghost, data_list);

}

fn print_formated(left: Vec<String>, right: Vec<String>) {
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
            print!("{}", right[i]);
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