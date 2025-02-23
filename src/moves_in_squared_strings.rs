fn diag_1_sym(s: &str) -> String {
    let diagonal_string_formatter: fn(acc: String, c: &str, i: usize) -> String =
        |acc, c, i| format!("{}{}", acc, c.chars().nth(i).unwrap()) ;

    rotate_string(s, diagonal_string_formatter)
}

fn rot_90_clock(s: &str) -> String {
    let rot_90_clock_string_formatter: fn(acc: String, c: &str, i: usize) -> String =
        |acc, c, i| format!("{}{}", c.chars().nth(i).unwrap(), acc);
    rotate_string(s, rot_90_clock_string_formatter)
}

fn rotate_string(
    s: &str,
    string_formatter: fn(acc: String, c: &str, i: usize) -> String,
) -> String {
    let mut res = vec![];
    let split = s.split("\n").collect::<Vec<_>>();

    for i in 0..split.len() {
        let new_string = split
            .iter()
            .fold(String::new(), |acc, &c| string_formatter(acc, c, i));
        res.push(new_string);
    }
    res.join("\n")
}

fn selfie_and_diag1(s: &str) -> String {
    let diagonal_result = diag_1_sym(s);
    let diagonal_split = diagonal_result.split("\n").collect::<Vec<_>>();

    s.split("\n")
        .zip(diagonal_split)
        .map(|a| format!("{}|{}", a.0, a.1))
        .collect::<Vec<_>>()
        .join("\n")
}

fn oper(my_func: fn(&str) -> String, s: &str) -> String {
    my_func(s)
}

#[cfg(test)]
mod tests {
    use super::{diag_1_sym, oper, rot_90_clock, selfie_and_diag1};

    fn testing_diag_1_sym(s: &str, exp: &str) {
        assert_eq!(oper(diag_1_sym, s), exp);
    }
    fn testing_rot_90_clock(s: &str, exp: &str) {
        assert_eq!(oper(rot_90_clock, s), exp);
    }
    fn testing_selfie_and_diag1(s: &str, exp: &str) {
        assert_eq!(oper(selfie_and_diag1, s), exp);
    }
    /*

    abcd\n
    efgh\n
    ijkl\n
    mnop

    diag_1_sym=
    aeim\n
    bfjn\n
    cgko\n
    dhlp

    rot_90_clock=
    miea\n
    njfb\n
    okgc\n
    plhd


     */
    #[test]
    fn sample_tests() {
        testing_diag_1_sym(
            "wuUyPC\neNHWxw\nehifmi\ntBTlFI\nvWNpdv\nIFkGjZ",
            "weetvI\nuNhBWF\nUHiTNk\nyWflpG\nPxmFdj\nCwiIvZ",
        );
        testing_diag_1_sym(
            "qAdPMX\nkRIQKU\nJeoroo\nNwVbtn\nAmQUqi\nVguxub",
            "qkJNAV\nARewmg\ndIoVQu\nPQrbUx\nMKotqu\nXUonib",
        );

        testing_rot_90_clock(
            "rgavce\nvGcEKl\ndChZVW\nxNWgXR\niJBYDO\nSdmEKb",
            "Sixdvr\ndJNCGg\nmBWhca\nEYgZEv\nKDXVKc\nbORWle",
        );
        testing_rot_90_clock(
            "EFAxSN\nXbJObC\nMrNVyg\nUKqDsE\nrYnAfU\nnNjADZ",
            "nrUMXE\nNYKrbF\njnqNJA\nAADVOx\nDfsybS\nZUEgCN",
        );

        testing_selfie_and_diag1("NJVGhr\nMObsvw\ntPhCtl\nsoEnhi\nrtQRLK\nzjliWg",
                                 "NJVGhr|NMtsrz\nMObsvw|JOPotj\ntPhCtl|VbhEQl\nsoEnhi|GsCnRi\nrtQRLK|hvthLW\nzjliWg|rwliKg");
        testing_selfie_and_diag1(
            "JAAn\nsrpa\nFngg\nmrVJ",
            "JAAn|JsFm\nsrpa|Arnr\nFngg|ApgV\nmrVJ|nagJ",
        );
    }
}
