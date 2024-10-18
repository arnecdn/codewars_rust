pub fn millipede(words: &[&str]) -> bool {
    milipede_rec(words.to_vec(), vec![])
}

fn milipede_rec<'a>(words: Vec<&'a str>, mut milipede: Vec<&'a str>) -> bool {
    if words.len() == 1 && milipede.last().unwrap().ends_with(words[0].chars().nth(0).unwrap()) {
        return true;
    }

    for i in 0..words.len() {
        if milipede.is_empty() || milipede.last().unwrap().ends_with(words[i].chars().nth(0).unwrap()) {
            milipede.push(words[i]);
            if milipede_rec([&words[..i], &words[i + 1..]].concat(), milipede.clone()) {
                return true;
            }
            milipede.pop();
        }
    }


    false
}


#[cfg(test)]
mod tests {
    use super::millipede;

    #[test]
    fn example_true() {
        //["excavate", "endure", "screen", "desire", "theater", "excess", "night"]
        // 1
        //"excavate",  "endure", "excess", "desire"
        // excavate, endure, excess
        // endure, excess
        // excess
        // desire, excavate, endure, excess

        //"excavate", "endure", "screen", "desire", "theater", "excess", "night"
        // excavate, endure, excess, screen
        // endure, excavate, excess, screen
        // screen, night
        // desire, excavate, endure, excess, screen, night, theatre


        // 2
        //
        // assert_eq!(millipede(&["", "", "", "", "", "", ""]), true);
        // assert_eq!(millipede(&["excavate", "desire", "fenduref"]), false);
        // assert_eq!(millipede(&["excavate", "endure"]), true);
        assert_eq!(millipede(&["excavate", "endure", "screen", "desire", "theater", "excess", "night"]), true);
    }

    #[test]
    fn example_false() {
        assert_eq!(millipede(&["trade", "pole", "view", "grave", "ladder", "mushroom", "president"]), false);
    }

    #[test]
    fn tests_are_not_broken() {
        assert_eq!(millipede(&["excavate", "east", "strike", "transport"]), true);
    }

    #[test]
    fn ah2023_false() {
        assert_eq!(millipede(&["stereotype", "evaluate", "stereotype", "empire"]), false);
    }

    #[test]
    fn five_words_true() {
        assert_eq!(millipede(&["screen", "desire", "theater", "excess", "night"]), true);
    }

    #[test]
    fn four_words_false() {
        assert_eq!(millipede(&["trade", "pole", "view", "grave"]), false);
    }
}