
fn string_to_industrial(time: &str) -> f64 {

    if let Some((hours, minutes)) = time.split_once(":") {
        let parsed_hours = hours.parse().unwrap_or(0) * 60;
        let parsed_minutes = minutes.parse().unwrap_or(0);
        return to_industrial(parsed_hours + parsed_minutes);
    }

    0.0
}

fn to_industrial(time: u32) -> f64 {
    format!("{:.2}", (time as f64 / 60.0)).parse::<f64>().unwrap_or(0.0)
}

fn to_normal(time: f64) -> String {
    format!("{}:{}", time.trunc(), format!("{:02}", ((time % 1.0) * 60.0).round()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(0.02==to_industrial(1), "Testing to_industrial()\nInput: {}, Expected: {}, Actual: {}", 1, 0.02, to_industrial(1));
        assert!(0.03==to_industrial(2), "Testing to_industrial()\nInput: {}, Expected: {}, Actual: {}", 2, 0.03, to_industrial(2));
        assert!(1.75==to_industrial(105), "Testing to_industrial()\nInput: {}, Expected: {}, Actual: {}", 105, 1.75, to_industrial(105));
        assert!(0.05==string_to_industrial("0:03"), "Testing string_to_industrial()\nInput: \"{}\", Expected: {}, Actual: {}", "0:03", 0.05, string_to_industrial("0:03"));
        assert!(0.07==string_to_industrial("0:04"), "Testing string_to_industrial()\nInput: \"{}\", Expected: {}, Actual: {}", "0:04", 0.07, string_to_industrial("0:04"));
        assert!(1.75==string_to_industrial("1:45"), "Testing string_to_industrial()\nInput: \"{}\", Expected: {}, Actual: {}", "1:45", 1.75, string_to_industrial("1:45"));
        assert!("1:45" == to_normal(1.75), "Testing to_normal()\nInput: {}, Expected: {}, Actual: {}", 1.75, "1:45", to_normal(1.75));
        assert!("0:20" == to_normal(0.33), "Testing to_normal()\nInput: {}, Expected: {}, Actual: {}", 0.33, "0:20", to_normal(0.33));
        assert!("106:04" == to_normal(106.07182078800426), "Testing to_normal()\nInput: {}, Expected: {}, Actual: {}", 106.07182078800426, "106:04", to_normal(106.07182078800426));
    }
}