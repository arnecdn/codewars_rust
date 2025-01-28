fn sjf(mut jobs: &[usize], index: usize) -> usize {
    let mut v = jobs.iter().enumerate().collect::<Vec<_>>();
    v.sort_by(|f1, f2| f1.1.cmp(f2.1));
    let mut total_cc = 0usize;

    for (job_id, &job_cc) in v {
        total_cc += job_cc;

        if job_id == index {
            return total_cc;
        }
    }

    0
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(sjf(&[100], 0), 100);
    assert_eq!(sjf(&[3, 10, 20, 1, 2], 0), 6);
    assert_eq!(sjf(&[3, 10, 20, 1, 2, 3], 5), 9);
}
