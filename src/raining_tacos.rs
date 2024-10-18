const TACOS: &str = "TACO";

fn rain_tacos(landscape: &str) -> String {
    let landscape_matrix = &landscape.split("\n").map(|s| String::from(s)).collect();

    let taco_cloud = create_taco_cloud(&landscape_matrix);

    let first_rotated_landscape = rotate(&landscape_matrix);
    let landscape_result = fill_landscape_from_left_to_right(taco_cloud, &first_rotated_landscape);
    let second_rotated_landscape = rotate(&landscape_result);

    let s = second_rotated_landscape.join("\n");
    s
}

fn create_taco_cloud(landscape_matrix: &Vec<String>) -> String {
    let landscape_width = landscape_matrix.first().unwrap().len();
    let taco_repetitions = (landscape_width as f32 / TACOS.len() as f32).ceil();
    TACOS.repeat(taco_repetitions as usize)[0..landscape_width].to_string()
}


fn rotate(landscape: &Vec<String>) -> Vec<String> {
    let mut rotaded_landscape = vec![];
    let landscape_width = landscape.first().unwrap().len();

    for i in 0..landscape_width {
        let layer = landscape.iter().fold(String::new(), |mut acc, s| {
            acc.push(s.chars().nth(i).unwrap());
            acc
        });

        rotaded_landscape.push(layer);
    }
    rotaded_landscape
}


fn fill_landscape_from_left_to_right(mut taco_cloud: String, landscape: &Vec<String>) -> Vec<String> {
    let mut landscape_result: Vec<String> = vec![];
    if landscape.is_empty() {
        landscape_result.push(String::from(""));
        return landscape_result;
    }

    for (i, v) in landscape.iter().enumerate() {
        let lanscape_part = v.chars().position(|c| c != ' ').or_else(|| Some(v.len())).unwrap();

        let mut res = v.clone();
        if lanscape_part > 0 && lanscape_part <= res.len() {
            let taco_char = taco_cloud.chars().nth(i).unwrap();
            taco_cloud.replace_range(i..i + 1, " ");

            res.replace_range(lanscape_part - 1..lanscape_part, &taco_char.to_string());
        }

        landscape_result.push(res);
    }
    landscape_result
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rain_tacos;

    fn dotest(landscape: &str, expected: &str) {
        let actual = rain_tacos(landscape);
        assert!(actual == expected,
                "With landscape = \"{}\"\nExpected \"{}\"\nGot \"{}\"", landscape, expected, actual)
    }

    #[test]
    fn fixed_tests() {
        dotest("", "");
        dotest(" ", "T");
        dotest(" *", "T*");
        dotest("     \n     \nOOOOO", "     \nTACOT\nOOOOO");
        dotest("OOOOO\nOOOOO\nOOOOO", "OOOOO\nOOOOO\nOOOOO");
        dotest("       \n       \n   O   \n  OOO  \n TACOS ", "       \n   O   \n  COT  \n AOOOA \nTTACOSC");
        dotest("* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *", "* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *\n* *\n*A*");
        dotest("          \n    ==    \n          \n          \n          ", "    TA    \n    ==    \n          \n          \nTACO  COTA");
    }
}
