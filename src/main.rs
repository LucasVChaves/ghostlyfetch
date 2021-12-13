use regex::{Captures, Regex};
use std::process::Command;
mod colours;

fn main() {
    let ascii_ghost = format!(
        "{bold_black}  ___   {reset}
{bold_black}    _/ ..\\   {reset}
{black}   ( \\  0/__   {reset}
{bold_white}    \\    \\__)  {reset}
{bold_white}    /     \\   {reset}
{white}   /      _\
{white}   `--------`   {reset}",
        bold_black = colours::bold_black,
        black = colours::black,
        bold_white = colours::bold_white,
        white = colours::white,
        reset = colours::reset
    );

    //Ghost found in https://www.asciiart.eu/mythology/ghosts
    let ascii_ghost = split_by_newline(ascii_ghost);

    let username = run_command("whoami", vec!());
    let hostname = run_command("cat", vec!("/etc/hostname"));
    let uptime = run_command("cat", vec!("/proc/uptime"));
    let kernel = run_command("uname", vec!("-mrs"));
    let memory = run_command("free", vec!("-m"));
    let distro_data = run_command("/bin/sh", vec!("-c", "cat /etc/*-release",));
    let shell = run_command("/bin/sh", vec!("-c", "echo $SHELL"));

    //Parsing the distro name
    let re_distro = match_regex(&distro_data, r#"(?x)DISTRIB_DESCRIPTION="?(?P<distro_name>[^\n"]+)"?\n"#.to_string());
    let distro_name = re_distro.name("distro_name").unwrap().as_str();

    //Parsing the shell name
    let re_shell = match_regex(&shell, r#"(?x)(?P<shell_name>[^/]+)$"#.to_string());
    let shell = re_shell.name("shell_name").unwrap().as_str();

    //Parsing the uptime in h:m format
    let re_uptime = match_regex(&uptime, r#"(?x)^(?P<uptime_seconds>\d+)\."#.to_string());
    let uptime_seconds: u32 = re_uptime.name("uptime_seconds").unwrap().as_str().parse().unwrap();
    let uptime_hours: u32 = uptime_seconds / 3600;
    let uptime_minutes: u32 = (uptime_seconds % 3600) / 60;

    //Parsing kernel
    let re_kernel = match_regex(&kernel, r#"(?x)(?P<kernel_name>\S+)\s+(?P<kernel_version>)\S+"#.to_string());
    let kernel = re_kernel.name("kernel_version").unwrap().as_str();

    //Parsing memory. Total and used RAM
    let re_memory = match_regex(&memory, r#"(?x)Mem:\s+(?P<total>\d+)\s+(?P<used>\d+)"#.to_string());
    let total_mem = re_memory.name("total").unwrap().as_str();
    let used_mem = re_memory.name("used").unwrap().as_str();

    let mut data_list: Vec<String> = Vec::new();
    let user_host_name = format!("{colour}{user}{reset}@{colour}{host}{reset}", 
        user = username,
        host = hostname,
        colour = colours::bold_red,
        reset = colours::reset).replace(" ", "").replace("\n", "");

    data_list.push(user_host_name);

    let mut separator = String::new();
    separator += colours::purple;
    for _i in 0..(username.len() + 1 + hostname.len()) {
        separator += "_";
    }

    data_list.push(separator);
    data_list.push(format_data("os", &distro_name));
    data_list.push(format_data("kernel", &kernel));
    data_list.push(format_data("shell", &shell));
    data_list.push(format_data("uptime", &format!("{hours}h:{minutes}m", hours = uptime_hours, minutes = uptime_minutes)));
    data_list.push(format_data("memory", &format!("{used}m / {total}m", used = used_mem, total = total_mem)));

    print_formated(ascii_ghost, data_list);

    //Splits a multi-line string into several strings. Separates by newline char
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
}

fn print_formated(left: Vec<String>, right: Vec<String>){
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {left_len} else {right_len};

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

fn format_data(key: &str, value: &str) -> String{
    return format!("{colour}▪{bold_colour} {key:7}{reset} {value}", 
        key = key,
        value = value,
        colour = colours::purple,
        bold_colour = colours::bold_purple,
        reset = colours::reset,);
}

//Return all matching regex's in a string
fn match_regex(search_str: &String, regex: String) -> Captures{
    let re = Regex::new(&regex).unwrap();

    return re.captures(&search_str).unwrap();
}

//Runs a command and returns the output of it
fn run_command(command: &str, args: Vec<&str>) -> String {
    let mut command = Command::new(command);

    command.args(args);

    let output = command.output().expect("Could not execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    return stdout.trim().to_string();
}
