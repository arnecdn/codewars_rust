fn finish_all(prerequisites: &[(u32, u32)]) -> bool {
    for job in prerequisites {
        let mut job_dependency = vec![];
        job_dependency.insert(0, job);

        for dependency in prerequisites {
            if job_dependency.last().unwrap().1 == dependency.0 {
                job_dependency.push(dependency);

                if job_dependency.first().unwrap().0 == job_dependency.first().unwrap().1 && job_dependency.first().unwrap().1 == job_dependency.last().unwrap().0 {
                    return false;
                } else if job_dependency.first().unwrap().0 == job_dependency.last().unwrap().1 && job_dependency.last().unwrap().1 == job_dependency.first().unwrap().0 {
                    return false;
                } else if job_dependency.first().unwrap().0 == job_dependency.last().unwrap().1 {
                    return false;
                }
            }
        }
    }
    return true;
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// Generate a function in Rust with fn finish_all_orig(prerequisites: &[(u32, u32)]) -> bool.


// The function is given a list of prerequisites needed to complete some jobs.
// The list contains pairs (j, i), where job j can start after job i has been completed.
// A job can have multiple dependencies or have no dependencies at all.
// If a job has no dependencies, it can be completed immediately.
// The function checks if all jobs can be finished and returns True if they can be completed,
// or False otherwise.
#[cfg(test)]
mod tests {
    use super::finish_all;

    fn dotest(prerequisites: &[(u32, u32)], expected: bool) {
        let actual = finish_all(prerequisites);
        assert!(actual == expected,
                "With prerequisites = {prerequisites:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        for (lst, expected) in
        [
            (&vec![(11, 10), (2, 11), (0, 11), (11, 6), (9, 2), (4, 2), (6, 4)], false),
            (&vec![(9, 2), (0, 10), (7, 9), (4, 7), (7, 4), (11, 4)], false),
            (&vec![(1, 9), (10, 8), (0, 0), (0, 9), (9, 8), (4, 10), (5, 9)], false),
            (&vec![(4, 1), (9, 6), (10, 1), (3, 9), (10, 6), (4, 4)], false),
            (&vec![(80, 73), (96, 16), (25, 30), (86, 42), (11, 54), (22, 64), (78, 79), (95, 75),
                   (15, 32), (49, 26), (93, 18), (13, 98), (39, 49), (88, 75), (89, 67), (21, 23),
                   (86, 32), (15, 15), (25, 25), (22, 81), (88, 16), (25, 55)], false),
            (&vec![(1, 0)], true),
            (&vec![(1, 0), (0, 1)], false),
            (&vec![(1, 0), (2, 1)], true),
            (&vec![(0, 1), (1, 2), (2, 0)], false),
            (&vec![], true),
            (&vec![(1, 2), (1, 0), (2, 0)], true),
            (&vec![(1, 0), (1, 2), (2, 0)], true),
            (&vec![(6, 10), (4, 3), (9, 2), (2, 3), (6, 1), (2, 8), (10, 1),
                   (10, 2), (5, 3), (0, 10), (7, 4)], true),
            (&vec![(7, 17), (4, 0), (20, 14), (12, 10), (4, 12), (14, 10),
                   (19, 0), (11, 5), (8, 18), (0, 17), (1, 12), (14, 6),
                   (14, 18), (4, 17), (9, 7), (19, 11), (9, 10), (1, 5),
                   (16, 6), (2, 13), (7, 9), (18, 18), (6, 16), (3, 14),
                   (0, 0), (11, 9), (7, 2), (13, 15), (16, 11)], false),
            (&vec![(3, 7), (18, 29), (27, 7), (28, 12), (9, 14), (2, 5), (5, 28),
                   (8, 27), (6, 5), (1, 24), (14, 3), (4, 8), (14, 12), (17, 11),
                   (3, 21), (5, 15), (10, 5), (29, 6), (11, 22), (13, 19), (29, 9),
                   (7, 6), (20, 12), (29, 21), (23, 14), (20, 21), (14, 5), (22, 21),
                   (17, 19), (27, 23), (13, 3), (27, 29), (10, 19), (10, 22)], true)
        ]

        {
            dotest(lst, expected)
        }
    }
}
