fn merge( line: &[u8]) -> Vec<u8> {
    let mut res = line.to_vec();

    for i in (0..res.len()){
        for j in i+1..res.len(){
            if res[i] == 0 {
                res[i] = res[j];
                res[j] = 0;
            }
            else if res[i] != res[j] && res[j] != 0{
                break;
            }
            else if res[i] == res[j] && res[i] != 0{
                res[i] = res[i] + res[j];
                res[j]=0;
                break;
            }
        }
    }
    res
}

fn merge_( line: &[u8]) -> Vec<u8> {

    let mut res = line.to_vec();

    'outer: for i in (0..res.len()){
        for j in i+1..res.len(){
            if res[i] == 0 && res[j] != 0 {
                res[i] = res[j];
                res[j] = 0;
            }
            if res[i] != res[j] && res[j] != 0{
                continue 'outer;
            }
            if res[i] == res[j] && res[i] != 0{
                res[i] = res[i] + res[j];
                res[j]=0;
                continue 'outer;
            }
        }

    }
    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::merge;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u8], expected: &[u8]) {
        assert_eq!(merge(a), expected, "{ERR_MSG} with line = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[2, 0, 2, 2], &[4, 2, 0, 0]);
        dotest(&[2, 0, 2, 4], &[4, 4, 0, 0]);
        dotest(&[0, 0, 2, 2], &[4, 0, 0, 0]);
        dotest(&[2, 2, 0, 0], &[4, 0, 0, 0]);
        dotest(&[2, 2, 2, 2, 2], &[4, 4, 2, 0, 0]);
        dotest(&[8, 16, 16, 8], &[8, 32, 8, 0]);
    }
}