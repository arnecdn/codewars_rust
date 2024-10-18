

fn add(num1: u32, num2: u32) -> u64 {
    let max_size = num1.to_string().len().max(num2.to_string().len());

    let numbers1_padded = format!("{:0>width$}", num1.to_string(), width = max_size);
    let numbers2_padded = format!("{:0>width$}", num2.to_string(), width = max_size);
    let mut total_padded = String::from("");

    for i in 0..max_size {
        let sum = numbers1_padded[i..i + 1].parse::<u64>().unwrap()
            + numbers2_padded[i..i + 1].parse::<u64>().unwrap();

        total_padded = format!("{}{}", sum, total_padded);
    }

    return total_padded.parse::<u64>().unwrap();
}

#[test]
fn test_real_add() {
    assert_eq!(add(2, 11), 13);
    assert_eq!(add(0, 1), 1);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_silly_add() {
    assert_eq!(add(16, 18), 214);
    assert_eq!(add(26, 39), 515);
    assert_eq!(add(122, 81), 1103);
}
