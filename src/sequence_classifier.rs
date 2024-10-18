use preloaded::Order;

mod preloaded {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Order {
        Unordered,
        Increasing,
        NotDecreasing,
        Decreasing,
        NotIncreasing,
        Constant,
    }
}



fn sequence_classifier(arr: &[i32]) -> Order {
    let mut state_computation = vec![];
    let mut state = Order::Unordered;


    for v in arr {
        if let Some(&last) = state_computation.last() {
            if last < v {
                if state == Order::Decreasing {
                    state = Order::Unordered;
                    break;
                } else if state == Order::Constant {
                    state = Order::NotDecreasing;
                    break;
                } else {
                    state = Order::Increasing;
                }
            } else if last > v {
                if state == Order::Increasing {
                    state = Order::Unordered;
                    break;
                } else if state == Order::Constant {
                    state = Order::NotIncreasing;
                    break;
                } else {
                    state = Order::Decreasing;
                }
            } else if last == v {
                if state == Order::Increasing {
                    state = Order::NotDecreasing;
                    break;
                } else if state == Order::Decreasing {
                    state = Order::NotIncreasing;
                    break;
                } else {
                    state = Order::Constant;
                }
            } else {
                state = Order::Constant;
            }
            state_computation.push(v);
        } else {
            state_computation.push(v);
        }
    }
    state
}


#[cfg(test)]
mod tests {
    use crate::sequence_classifier::sequence_classifier;
    use super::preloaded::{Order, Order::*};

    #[test]
    fn sample_tests() {
        let cases: [(&[i32], Order); 8] = [
            // (&[3, 5, 8, 1, 14, 3], Unordered),
            // (&[3, 5, 8, 9, 14, 23], Increasing),
            // (&[3, 5, 8, 8, 14, 14], NotDecreasing),
            (&[14, 9, 8, 5, 3, 1], Decreasing),
            (&[14, 14, 8, 8, 5, 3], NotIncreasing),
            (&[8, 8, 8, 8, 8, 8], Constant),
            (&[8, 9], Increasing),
            (&[8, 8, 8, 8, 8, 9], NotDecreasing),
            (&[9, 8], Decreasing),
            (&[9, 9, 9, 8, 8, 8], NotIncreasing),
            (&[3, 5, 8, 1, 14, 2], Unordered),
        ];
        for (seq, expected) in cases {
            let actual = sequence_classifier(seq);
            assert_eq!(actual, expected, "\nYour result (left) did not match the expected output (right) for the sequence:\n{seq:?}");
        }
    }
}
