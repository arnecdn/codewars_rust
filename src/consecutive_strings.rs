
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k <= 0 || strarr.len() < k || strarr.len() <= 0 { return String::new(); }

    strarr[..strarr.len() - k + 1].iter().enumerate().fold(String::new(), | acc, w| -> String{
        let consec = strarr[w.0..w.0 + k].join("");
        if consec.len() > acc.len() { consec } else { acc }
    })
}


fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["bbxxxyyyeesss", "qqzzef", "rrjdddwww", "ffnnnaaanw", "bbppykfll", "dddlllbblluuuyy", "ejjjffoo", "uuuebbgdxxx", "rrrppppppijvvvgggww", "ppbbbkkkaaaxxxfrrhhhttt"], 3, "uuuebbgdxxxrrrppppppijvvvgggwwppbbbkkkaaaxxxfrrhhhttt");
    testing(vec!["llloosssulx", "bkkkiiigaaqq", "iifffaaad", "wwhaa", "izzziifpppk", "uuufffllll", "rrrskkkl", "tmfpmmlll", "ffftfffdnnn"], 8, "llloosssulxbkkkiiigaaqqiifffaaadwwhaaizzziifpppkuuufffllllrrrskkkltmfpmmlll");
    //     "dddlllbblluuuyy rrrppppppijvvvgggwwppbbbkkkaaaxxxfrrhhhttt"
    //     "uuuebbgdxxx     rrrppppppijvvvgggwwppbbbkkkaaaxxxfrrhhhttt"
}

// fn longest_consec_old(strarr: Vec<&str>, k: usize) -> String {
//     if k <= 0 || strarr.len() < k || strarr.len() <= 0 { return String::new(); }
//     println!("strarr{:?}, k{}",strarr,k);
//     if k == 1 { return strarr.iter().max_by(|x, x1| { x.len().cmp(&x1.len()) }).unwrap().to_string(); }
//
//     strarr.iter().enumerate().fold(vec![], |acc, w| -> Vec<_>{
//         let s = strarr[w.0 + 1..]
//             .chunks(k - 1)
//             .map(|c|  format!("{}{}",w.1 ,c.join("")))
//             .collect::<Vec<_>>();
//         [acc,s].concat()
//
//     }).iter()
//         .max_by(|x, x1| { x.len().cmp(&x1.len()) })
//         .unwrap().to_string()
// }
