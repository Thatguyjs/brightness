mod brightness;
use brightness::*;

// use clap::{Arg, Command};


fn print_help() {
    println!("brightness v{}\n\nHelp manual:\n{}\nExamples:\n{}",
        env!("CARGO_PKG_VERSION"),
        "  --help  -h          Display this menu\n  \
           --info  -i          Display monitor brightness info\n  \
           [+|-](number)[%]    Add, subtract, or set the brightness value or percentage to (number)\n  \
           min                 Set the brightness to the lowest value (1%)\n  \
           max                 Set the brightness to the highest value (100%)\n\
        ",
        "  brightness +10%\n  \
           brightness +250\n  \
           brightness -15%\n  \
           brightness -475\n  \
           brightness 50%\n  \
           brightness 9600\n  \
           brightness min\n  \
           brightness max\
        "
    );
}

fn print_info() {
    let max = get_max_brightness().expect("Failed to get max brightness");

    println!("brightness v{}\n\nMinimum brightness (1%):   {}\nMaximum brightness (100%): {}",
        env!("CARGO_PKG_VERSION"),
        max / 100,
        max
    );
}


fn main() {
    let mut arg = std::env::args().skip(1).next().unwrap_or("".into());

    if arg.is_empty() || arg == "--help" || arg == "-h" {
        print_help();
        std::process::exit(0);
    }
    else if arg == "--info" || arg == "-i" {
        print_info();
        std::process::exit(0);
    }

    let percentage =
        if arg.ends_with('%') {
            arg.pop();
            true
        }
        else {
            false
        };

    if arg == "min" {
        arg = "0".into();
    }
    else if arg == "max" {
        match percentage {
            true => arg = "100".into(),
            false => arg = get_max_brightness().unwrap().to_string()
        }
    }

    let arg_val: i32 =
        match arg.chars().next() {
            Some(ch) if ch == '-' || ch.is_digit(10) =>
                arg.parse().unwrap_or(0),
            _ =>
                arg[1..].parse().unwrap_or(0)
        };

    if arg.starts_with('+') {
        match percentage {
            true => change_brightness_percent(arg_val as f32).unwrap(),
            false => change_brightness(arg_val).unwrap()
        }
    }
    else if arg.starts_with('-') {
        match percentage {
            true => change_brightness_percent(arg_val as f32).unwrap(),
            false => change_brightness(arg_val).unwrap()
        }
    }
    else if arg.chars().all(|c| c.is_digit(10)) {
        match percentage {
            true => set_brightness_percent(arg_val as f32).unwrap(),
            false => set_brightness(arg_val).unwrap()
        }
    }
    else {
        print_help();
    }
}
