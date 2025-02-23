
use std::fmt::{Display};
use std::time::Duration;

fn stati(strg: &str) -> String {
    if strg.is_empty() {
        return "".to_string();
    }

    let mut durations = parse_durations_from_list(strg);
    durations.sort();

    let range = compute_range(&durations);
    let avg = compute_average_duration(&durations);
    let median = compute_median(&durations);

    format!(
        "Range: {} Average: {} Median: {}",
        DisplayDuration(range),
        DisplayDuration(avg),
        DisplayDuration(median)
    )
}

fn parse_durations_from_list(strg: &str) -> Vec<Duration> {
    strg.split(",")
        .map(|e| parse_duration(e))
        .collect()
}

fn parse_duration(e: &str) -> Duration {
    let timed = e.split("|").collect::<Vec<_>>();

    let timed_tuple = match timed[..] {
        [hours, mins, secs] => (
            hours.trim().parse::<i32>().unwrap(),
            mins.trim().parse::<i32>().unwrap(),
            secs.trim().parse::<i32>().unwrap(),
        ),
        _ => return panic!("Invalid timeformat. Expected timed format: hh|mm|ss"),
    };

    Duration::new(
        (timed_tuple.0 * 3600 + timed_tuple.1 * 60 + timed_tuple.2) as u64,
        0,
    )
}
fn compute_range(durations: &Vec<Duration>) -> Duration {
    *durations.iter().max().unwrap() - *durations.iter().min().unwrap()
}

fn compute_median(durations: &Vec<Duration>) -> Duration {
    if durations.len() % 2 == 0 {
        return (durations[durations.len() / 2 - 1] + durations[durations.len() / 2]) / 2;
    }
    durations[durations.len() / 2]
}

fn compute_average_duration(durations: &Vec<Duration>) -> Duration {
    durations.iter().sum::<Duration>() / durations.len() as u32
}


struct DisplayDuration(Duration);

impl Display for DisplayDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.0.as_secs() / 3600;
        let mins = (self.0.as_secs() % 3600) / 60;
        let secs = self.0.as_secs() % 60;

        write!(f, "{:0>2}|{:0>2}|{:0>2}", hours, mins, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(strg: &str, exp: &str) -> () {
        println!(" str: {:?};", strg);
        let ans = stati(strg);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(
            "01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17",
            "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34",
        );
        dotest(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41",
            "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00",
        );
        dotest(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|32|34, 2|17|17",
            "Range: 00|31|17 Average: 02|27|10 Median: 02|24|57",
        );
        dotest(
            "",
            "",
        );
    }



}
