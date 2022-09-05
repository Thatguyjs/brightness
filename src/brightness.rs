// Brightness utility functions


use std::{io, fs};


fn limit<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if min > max {
        panic!("limit(): 'min' cannot be larger than 'max'!");
    }

    match value {
        v if v < min => min,
        v if v > max => max,
        v => v
    }
}


pub fn get_brightness() -> io::Result<i32> {
    let value = fs::read_to_string("/sys/class/backlight/intel_backlight/brightness")?;
    Ok(value.trim().parse().unwrap_or(-1))
}

pub fn get_max_brightness() -> io::Result<i32> {
    let value = fs::read_to_string("/sys/class/backlight/intel_backlight/max_brightness")?;
    Ok(value.trim().parse().unwrap_or(-1))
}

pub fn get_brightness_percent() -> io::Result<f32> {
    let actual = get_brightness()? as f32;
    let max = get_max_brightness()? as f32;

    Ok(100f32 * (actual + 0.5) / max)
}


pub fn set_brightness(value: i32) -> io::Result<()> {
    let max = get_max_brightness()?;
    let value = limit(value, max / 100, max);

    // Needs root
    fs::write("/sys/class/backlight/intel_backlight/brightness", value.to_string())
}

pub fn change_brightness(change: i32) -> io::Result<()> {
    set_brightness(get_brightness()? + change)
}

pub fn set_brightness_percent(percent: f32) -> io::Result<()> {
    let max = get_max_brightness()?;

    let percent = limit(percent, 1.0, 100.0);
    let value = (percent / 100f32 * (max as f32) + 0.5) as i32;

    set_brightness(limit(value, max / 100, max))
}

pub fn change_brightness_percent(change: f32) -> io::Result<()> {
    set_brightness_percent(get_brightness_percent()? + change)
}
