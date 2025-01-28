use num::Integer;

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odd_arr = arr.iter().filter(|c| c.is_odd()).collect::<Vec<_>>();
    odd_arr.sort();
    odd_arr.reverse();

    let res = arr.iter().fold(vec![],|mut acc, e| -> Vec<_>{
        if e.is_odd() {acc.push(*odd_arr.pop().unwrap());
        }else { acc.push(*e)}
        acc
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}