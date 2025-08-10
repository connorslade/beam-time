const TIME_UNITS: &[(u64, &str)] = &[(86400, "d"), (3600, "h"), (60, "m"), (1, "s")];

pub fn human_duration(mut secs: u64) -> String {
    let mut out = String::new();

    if secs == 0 {
        return "0s".into();
    }

    for &(unit, label) in TIME_UNITS {
        if secs >= unit {
            out.push_str(&format!("{}{} ", secs / unit, label));
            secs %= unit;
        }
    }

    out.trim_end().to_string()
}

pub fn human_duration_minimal(secs: u64) -> String {
    if secs == 0 {
        return "0s".into();
    }

    for &(unit, label) in TIME_UNITS {
        if secs >= unit {
            return format!("{}{}", secs / unit, label);
        }
    }

    let (unit, label) = TIME_UNITS[0];
    format!("{}{}", secs / unit, label)
}
