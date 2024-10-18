fn generate_shape(n: i32) -> String {
    vec![String::from("+".repeat(n as usize)); n as usize].join("\n")
}


#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
