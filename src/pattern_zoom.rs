fn zoom(n: i32) -> String {
    let decorate = |source: char| -> String { if source == '■' { String::from("□") } else { String::from("■") } };

    if n == 1 {
        return String::from("■");
    }

    let square = zoom(n - 1);

    if n % 2 == 0 {
        return square;
    }
    let last_layer = square.chars().nth(0).unwrap();
    let  inner_sqare = square.split("\n").fold(vec![], |mut acc, s| {
        acc.push(format!("{}{}{}", decorate(last_layer), s, decorate(last_layer)));
        acc
    });

    format!("{}\n{}\n{}", vec![decorate(last_layer); n as usize].join(""), inner_sqare.join("\n"), vec![decorate(last_layer); n as usize].join(""))
}

#[test]
fn basic_test_1() {
    assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
    assert_eq!(zoom(3), "\
□□□
□■□
□□□"
    );
}

#[test]
fn basic_test_3() {
    assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
    );
}

#[test]
fn basic_test_4() {
    assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
    );
}

#[test]
fn basic_test_5() {
    assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
    );
}