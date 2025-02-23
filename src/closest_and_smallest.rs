fn closest(s: &str) -> String {
    if s.is_empty() {
        return "[]".to_string();
    }
    let mut weighted = weighted_numbers(s);

    let (shortest_current, shortest_neighbour) = find_closest_wheighted_tuple(&mut weighted);

    format!("[{:?}{:?}]", shortest_current, shortest_neighbour).replace(" ", "")
}

fn find_closest_wheighted_tuple(weighted: &mut Vec<(u32, usize, u32)>) -> ((u32, usize, u32), (u32, usize, u32)) {
    weighted.sort_by_key(|a| a.0);

    let (mut shortest_current, mut shortest_neighbour) = (weighted[0], weighted[1]);
    let mut current = shortest_current;

    for next in &mut weighted[1..] {
        if current.0.abs_diff(next.0) < shortest_current.0.abs_diff(shortest_neighbour.0) {
            shortest_current = current;
            shortest_neighbour = *next;
        }
        current = *next;
    }
    (shortest_current, shortest_neighbour)
}

fn weighted_numbers(s: &str) -> Vec<(u32, usize, u32)> {
    let mut weighted = s
        .trim()
        .split_whitespace()
        .enumerate()
        .map(|(i, n)| {
            let weight = n.chars().filter_map(|c| c.to_digit(10)).sum::<u32>();
            (weight, i, n.parse::<u32>().unwrap_or(0))
        })
        .collect::<Vec<_>>();
    weighted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s: &str, exp: String) -> () {
        let ans = closest(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }

    #[test]
    fn basic_tests() {
        let mut s = "503007 312 794196 539 766478 677 63172 83 696442 33 210219 582 444364 78 802297 388 632946 235 465227 411 410879 223 81463 969";
        testing(s, "[(6,1,312)(6,9,33)]".to_string());

        let mut s = "456899 50 11992 176 272293 163 389128 96 290193 85 52"; // [(13, 9, "85"), (14, 3, "176")]
        testing(s, "[(13,9,85)(14,3,176)]".to_string());

        s = "239382 162 254765 182 485944 134 468751 62 49780 108 54"; // "[[8, 5, 134], [8, 7, 62]]";
        testing(s, "[(8,5,134)(8,7,62)]".to_string());

        s = "241259 154 155206 194 180502 147 300751 200 406683 37 57";
        let r = "[(10,1,154)(10,9,37)]";
        testing(s, r.to_string());
    }
}
