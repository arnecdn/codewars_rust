fn expanded_form(n: u64) -> String {
    n.to_string().char_indices().fold(vec![],|mut acc,t|-> Vec<_>{
        if t.1 != '0' { acc.push(format!("{:0<width$}", t.1, width = n.to_string().len() - t.0).trim().to_string()); }
        acc
    }).join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}