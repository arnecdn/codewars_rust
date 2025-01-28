fn dup(arry: Vec<String>) -> Vec<String> {
    arry.iter()
        .map(|w| {
            w.chars().fold(String::new(), |mut acc, c| -> String {
                if acc.is_empty() || Some(c) != acc.chars().last() {
                    acc.push(c)
                }

                acc
            })
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        // match a list of expressions separated by comma:
        ($($str:expr),*) => ({
            // create a Vec with this list of expressions,
            // calling String::from on each:
            vec![$(String::from($str),)*] as Vec<String>
        });
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            dup(vec_of_strings![
                "ccooddddddewwwaaaaarrrrsssss",
                "piccaninny",
                "hubbubbubboo"
            ]),
            vec!["codewars", "picaniny", "hubububo"]
        );
        assert_eq!(
            dup(vec_of_strings!["abracadabra", "allottee", "assessee"]),
            vec!["abracadabra", "alote", "asese"]
        );
        assert_eq!(
            dup(vec_of_strings!["kelless", "keenness"]),
            vec!["keles", "kenes"]
        );
        assert_eq!(
            dup(vec_of_strings![
                "Woolloomooloo",
                "flooddoorroommoonlighters",
                "chuchchi"
            ]),
            vec!["Wolomolo", "flodoromonlighters", "chuchchi"]
        );
        assert_eq!(
            dup(vec_of_strings!["adanac", "soonness", "toolless", "ppellee"]),
            vec!["adanac", "sones", "toles", "pele"]
        );
        assert_eq!(
            dup(vec_of_strings!["callalloo", "feelless", "heelless"]),
            vec!["calalo", "feles", "heles"]
        );
        assert_eq!(
            dup(vec_of_strings!["putteellinen", "keenness"]),
            vec!["putelinen", "kenes"]
        );
        assert_eq!(
            dup(vec_of_strings!["kelless", "voorraaddoosspullen", "achcha"]),
            vec!["keles", "voradospulen", "achcha"]
        );
    }
}
