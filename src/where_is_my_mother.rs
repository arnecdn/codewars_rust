use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{format, Display};
use std::iter::{FromIterator, Rev};

fn find_children(dancing_brigade: &str) -> String {
    let mut db = dancing_brigade
        .chars()
        .collect::<Vec<_>>();

    db.sort_by(|&f1, &f2| sort_by_family(f1, f2));

     db.iter().collect::<String>()
}

fn sort_by_family(first:char, other: char) -> Ordering {
    if first.is_ascii_uppercase()
        && other.is_ascii_lowercase()
        && first.to_lowercase().eq(other.to_lowercase())
    {
        return Ordering::Less;
    } else if first.is_ascii_lowercase()
        && other.is_ascii_uppercase()
        && first.to_lowercase().eq(other.to_lowercase())
    {
        return Ordering::Greater;
    }
    first.to_lowercase().cmp(&mut other.to_lowercase())
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_children;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn sample_tests() {
        assert_eq!(find_children("abBA"), "AaBb", "{}", ERR_MSG);
        assert_eq!(find_children("AaaaaZazzz"), "AaaaaaZzzz", "{}", ERR_MSG);
        assert_eq!(find_children("CbcBcbaA"), "AaBbbCcc", "{}", ERR_MSG);
        assert_eq!(find_children("xXfuUuuF"), "FfUuuuXx", "{}", ERR_MSG);
        assert_eq!(find_children(""), "", "{}", ERR_MSG);
    }
}
