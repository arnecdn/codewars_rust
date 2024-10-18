fn tower_builder(n_floors: usize, block_size: (usize, usize)) -> Vec<String> {
    let (block_width, block_height) = block_size;
    let calc_width = |floor| -> usize { block_width + (block_width * (floor - 1) * 2) };

    (1..=n_floors).fold(vec![], | acc, i| {
        [acc, vec![format!("{:^width$}", &"*".repeat(calc_width(i)), width = calc_width(n_floors)); block_height]].concat()
    },
    )
}

// **********************
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
//         assert_eq!(tower_builder(6, (2, 1)), vec![
//   "          **          ",
//   "        ******        ",
//   "      **********      ",
//   "    **************    ",
//   "  ******************  ",
//   "**********************"
// ]
//         );
        // assert_eq!(tower_builder(1, (1, 1)), vec!["*"]);
        assert_eq!(tower_builder(3, (4, 2)), vec![
            "        ****        ",
            "        ****        ",
            "    ************    ",
            "    ************    ",
            "********************",
            "********************"]);
    }
}
