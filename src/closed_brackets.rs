fn closed_brackets(s: &str) -> bool {
    if s.is_empty() || s == "J" {
        return true;
    }
    if Some(')') == s.chars().nth(0) {
        return false;
    }


    let mut open_brackets = vec![];

    for bracket in s.chars() {
        if bracket == '(' || bracket== 'J' {
            open_brackets.push(bracket);
            continue;
        }

        if let Some(&open) = open_brackets.last() {
            if bracket == ')' && (open == '(' || open == 'J') {
                open_brackets.pop();
            } else if bracket == 'J' && (open == '(' || open == 'J') {
                open_brackets.pop();
            }
        }
    }

    open_brackets.is_empty()
}

#[cfg(test)]
mod tests {
    use super::closed_brackets;
    // ()
    // ()
    // (())
    // ()
    // ()
    //
    // J(((JJ(J((J)))(((((JJ)JJJ(J))()J
    #[test]
    #[ignore]
    fn fixed_tests() {
        dotest("JJ(J)(J))JJ)J()(((J)J)(JJJJ)JJJ())(JJ", true);

        dotest("(J)(J))JJ)J()(((J)J)(JJJJ)JJJ())(JJ", true);
        dotest("JJJJ((J)()J)J(((JJ(J((J)))(((((JJ)JJJ(J))()J", true);
        // dotest("(J))", true);
        // dotest("(", false);
        // dotest("", true);
        // dotest("()J(", false);
        // dotest("J", true);
        // dotest(")(", false);
        // dotest("()", true);
        // dotest("J)(J", true);
        // dotest("(J()J(()(J", false);
        // dotest("J(JJ()J((J", false);
        // dotest("))", false);
    }

    fn dotest(s: &str, expected: bool) {
        let actual = closed_brackets(s);
        assert!(actual == expected,
                "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }
}
