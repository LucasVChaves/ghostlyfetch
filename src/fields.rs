#![allow(clippy::needless_return)]

use crate::colours;
use regex::{Captures, Regex};
use std::env;
use std::fs;
use std::io::Read;

fn format_data(key: &str, value: &str) -> String {
    return format!(
        "{colour}▪{bold_colour} {key:7}{reset} {value}",
        key = key,
        value = value,
        colour = colours::purple,
        bold_colour = colours::bold_purple,
        reset = colours::reset,
    );
}

//Return all matching regex's in a string
fn match_regex(search_str: &str, regex: String) -> Option<Captures> {
    let re = Regex::new(&regex).unwrap();

    return re.captures(search_str)
}

pub fn get_user_host_name(is_halloween: bool) -> Result<(String, String), String> {
    let username_env = env::var_os("USER");
    let username: String;
    if username_env.is_some() {
        username = username_env.unwrap().into_string().unwrap();
    } else {
        username = String::new();
    }

    let hostname_file = fs::File::open("/etc/hostname");
    if hostname_file.is_err() {
        return Err("error".to_string());
    }
    let mut hostname_file = hostname_file.unwrap();
    let mut hostname = String::new();
    let result = hostname_file.read_to_string(&mut hostname);

    if result.is_err() {
        return Err("error".to_string());
    }

    let main_colour: &str;

    if is_halloween {
        main_colour = colours::bold_red;
    } else {
        main_colour = colours::purple;
    }

    let user_host_name = format!(
        "{color}{user}{reset}@{color}{host}{reset}",
        user = username,
        host = hostname,
        color = main_colour,
        reset = colours::reset,
    )
    .replace(" ", "")
    .replace("\n", "");

    let user_host_name_len = username.len() + 1 + hostname.len();
    let mut separator = String::new();

    if is_halloween {
        separator += colours::bold_yellow;
    } else {
        separator += colours::bold_white;
    }

    for _i in 0..(user_host_name_len) {
        separator += "━";
    }
    separator += colours::reset;

    return Ok((user_host_name, separator));
}

pub fn get_distro_name() -> Result<String, String> {
    let lsb_release = fs::File::open("/etc/lsb-release");
    let mut buffer = String::new();

    if lsb_release.is_ok() {
        let mut lsb_release = lsb_release.unwrap();
        let result = lsb_release.read_to_string(&mut buffer);

        if result.is_err() {
            return Err("error".to_string());
        }

        let re_lsb = match_regex(
            &buffer,
            r#"(?x)DISTRIB_DESCRIPTION="?(?P<distro_name>[^\n"]+)"?\n"#.to_string(),
        );

        if re_lsb.is_some() {
            let re_lsb = re_lsb.unwrap();
            let distro_name = re_lsb.name("distro_name").unwrap().as_str();

            return Ok(format_data("os", &distro_name));
        }
    }

    let os_release = fs::File::open("/etc/os-release");

    if os_release.is_err() {
        return Err("error".to_string());
    }

    let mut os_release = os_release.unwrap();
    let result = os_release.read_to_string(&mut buffer);

    if result.is_err() {
        return Err("error".to_string());
    }

    let re_os = match_regex(
        &buffer,
        r#"(?x)PRETTY_NAME="?(?P<distro_name>[^\n"]+"?\n)"#.to_string(),
    );

    if re_os.is_some() {
        let re_os = re_os.unwrap();

        let distro_name = re_os.name("distro_name").unwrap().as_str();
        return Ok(format_data("os", &distro_name));
    }

    return Err("error".to_string());
}

pub fn get_kernel() -> Result<String, String> {
    let kernel_file = fs::File::open("/proc/version");

    if kernel_file.is_err() {
        return Err("Error".to_string());
    }

    let mut kernel_file = kernel_file.unwrap();
    let mut kernel = String::new();

    let result = kernel_file.read_to_string(&mut kernel);

    if result.is_err() {
        return Err("Error".to_string());
    }

    let re_kernel = match_regex(
        &kernel,
        r#"(?x)Linux\sversion\s(?P<kernel_version>\S+)"#.to_string(),
    );

    if re_kernel.is_none() {
        return Err("Error".to_string());
    }
    
    let re_kernel = re_kernel.unwrap();
    
    let kernel = re_kernel.name("kernel_version").unwrap().as_str();
    return Ok(format_data("kernel", &kernel));
}

pub fn get_shell() -> Result<String, String> {
    let shell_env = env::var_os("SHELL");

    if shell_env.is_none() {
        return Err("Error".to_string());
    }

    let shell = shell_env.unwrap().into_string().unwrap();

    let re_shell = match_regex(&shell, r#"(?x)(?P<shell_name>[^/]+)$"#.to_string());

    if re_shell.is_none() {
        return Err("Error".to_string());
    }

    let re_shell = re_shell.unwrap();

    let shell = re_shell.name("shell_name").unwrap().as_str();
    return Ok(format_data("shell", &shell));
}

pub fn format_uptime(time: std::time::Duration) -> String {
    let uptime_seconds = time.as_secs();

    let uptime_hours = uptime_seconds / (60 * 60);
    let uptime_minutes = (uptime_seconds % (60 * 60)) / 60;

    format_data("uptime", &format!("{hours}h and {minutes}m",
        hours = uptime_hours,
        minutes = uptime_minutes))
}

pub fn format_memory(mem: systemstat::Memory) -> String {
    format_data("memory", &format!("{used} / {total}",
        used = systemstat::saturating_sub_bytes(mem.total, mem.free),
        total = mem.total))
}

pub fn format_battery(battery: systemstat::BatteryLife) -> String {
    format_data("battery", &format!("{percent:.1}%, {hours}h and {minutes}m remaining",
        percent = battery.remaining_capacity * 100.0,
        hours = battery.remaining_time.as_secs() / (60 * 60),
        minutes = battery.remaining_time.as_secs() % 60))
    
}