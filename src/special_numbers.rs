

fn special_number(n: u64) -> String {
    let special_numbers = ['0','1','2','3','4','5'];
    if n.to_string().as_str().chars().all(|c| special_numbers.contains(&c)){
        return "Special!!".to_string();
    }else {
        return  "NOT!!".to_string();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(special_number(2),"Special!!");
        assert_eq!(special_number(3),"Special!!");
        assert_eq!(special_number(6),"NOT!!");
        assert_eq!(special_number(9),"NOT!!");
        assert_eq!(special_number(11),"Special!!");
        assert_eq!(special_number(55),"Special!!");
        assert_eq!(special_number(26),"NOT!!");
        assert_eq!(special_number(92),"NOT!!");
        assert_eq!(special_number(25432),"Special!!");
        assert_eq!(special_number(2783),"NOT!!");
    }
}