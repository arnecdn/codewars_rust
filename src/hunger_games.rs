use std::collections::{HashMap, HashSet};



fn get_eating_rules<'a>() -> HashMap<&'static str, HashSet<&'static str>> {
    HashMap::from([
        ("antelope", HashSet::from(["grass"])),
        ("bug", HashSet::from(["leaves"])),
        ("big-fish", HashSet::from(["little-fish"])),
        ("bear", HashSet::from(["big-fish", "bug", "chicken", "cow", "leaves", "sheep"])),
        ("chicken", HashSet::from(["bug"])),
        ("cow", HashSet::from(["grass"])),
        ("fox", HashSet::from(["chicken", "sheep"])),
        ("giraffe", HashSet::from(["leaves"])),
        ("lion", HashSet::from(["antelope", "cow"])),
        ("panda", HashSet::from(["leaves"])),
        ("sheep", HashSet::from(["grass"])),
    ])
}


fn who_eats_who(zoo: &str) -> Vec<String> {
    let animals = zoo.split(",").collect::<Vec<_>>();
    return who_eats_who_rec(animals, vec![String::from(zoo)]);
}

fn who_eats_who_rec<'a>(mut animals: Vec<&'a str>, mut result: Vec<String>) -> Vec<String> {
    let eating_rules = get_eating_rules();

    for (i, animal) in animals.iter().enumerate() {
        if let Some(animal_eating_rule) = eating_rules.get(animal) {
            if i > 0 {
                let food_left = animals[i - 1];

                if animal_eating_rule.get(food_left).is_some() {
                    result.push(format!("{} eats {}", animal, food_left));
                    animals.remove(i - 1);

                    return who_eats_who_rec(animals, result);
                }
            }

            if i < animals.len() - 1 {
                let food_right = animals[i + 1];

                if animal_eating_rule.get(food_right).is_some() {
                    result.push(format!("{} eats {}", animal, food_right));
                    animals.remove(i + 1);

                    return who_eats_who_rec(animals, result);
                }
            }
        }
    }

    result.push(animals.join(","));
    result
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::who_eats_who;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: [&str; 6]) {
        assert_eq!(who_eats_who(s), expected, "{ERR_MSG} with zoo = \"{s}\"")
    }


    // ["bear,grass,grass,grass,grass,sheep,bug,chicken,little-fish,little-fish,little-fish,little-fish,big-fish,big-fish,big-fish","sheep eats grass", "sheep eats grass", "sheep eats grass", "sheep eats grass", "bear eats sheep", "bear eats bug", "bear eats chicken", "big-fish eats little-fish", "big-fish eats little-fish", "big-fish eats little-fish", "big-fish eats little-fish", "bear eats big-fish", "bear eats big-fish", "bear eats big-fish", "bear"]`
    #[test]
    fn example_test_1() {
        assert_eq!(who_eats_who("bear,grass,grass,grass,grass,sheep,bug,chicken,little-fish,little-fish,little-fish,little-fish,big-fish,big-fish,big-fish"),
                   ["bear,grass,grass,grass,grass,sheep,bug,chicken,little-fish,little-fish,little-fish,little-fish,big-fish,big-fish,big-fish", "sheep eats grass", "sheep eats grass", "sheep eats grass", "sheep eats grass", "bear eats sheep", "bear eats bug", "bear eats chicken", "big-fish eats little-fish", "big-fish eats little-fish", "big-fish eats little-fish", "big-fish eats little-fish", "bear eats big-fish", "bear eats big-fish", "bear eats big-fish", "bear"]);
    }

    #[test]
    fn example_test_2() {
        assert_eq!(who_eats_who("grass,grass,cow,leaves,bug,big-fish,leaves,bear"),
                   ["grass,grass,cow,leaves,bug,big-fish,leaves,bear", "cow eats grass", "cow eats grass", "bug eats leaves", "bear eats leaves", "bear eats big-fish", "bear eats bug", "bear eats cow", "bear"]);
    }

    #[test]
    fn example_test_3() {
        assert_eq!(who_eats_who("chicken,fox,leaves,bug,grass,sheep"),
                   ["chicken,fox,leaves,bug,grass,sheep", "fox eats chicken", "bug eats leaves", "sheep eats grass", "fox,bug,sheep"]);
    }

    #[test]
    fn example_test_4() {
        assert_eq!(who_eats_who("giraffe,leaves,leaves,leaves,bear,bug,leaves,leaves,panda"),
                   ["giraffe,leaves,leaves,leaves,bear,bug,leaves,leaves,panda", "giraffe eats leaves", "giraffe eats leaves", "giraffe eats leaves", "bear eats bug", "bear eats leaves", "bear eats leaves", "giraffe,bear,panda"]);
    }

    #[test]
    fn example_test() {
        dotest("fox,bug,chicken,grass,sheep", ["fox,bug,chicken,grass,sheep",
            "chicken eats bug",
            "fox eats chicken",
            "sheep eats grass",
            "fox eats sheep",
            "fox"]);
    }
}