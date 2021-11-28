
fn generate_shape(n: i32) -> String {
  return "+".repeat((n*n)as usize).chars()
  .collect::<Vec<char>>()
  .chunks(n as usize)
  .map(|c| c.iter().collect::<String>())
  .collect::<Vec<String>>()
  .join("\n");

}

fn generate_shape_best(n: i32) -> String {
  return vec!["+".repeat(n as usize); n as usize].join("\n");
}

#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
