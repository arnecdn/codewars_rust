fn rgb(r: i32, g: i32, b: i32) -> String {
    [r,g,b].iter().map(|s| format!("{:02X}", s.clamp(&0, &255))).collect()
    // format!("{:02X}{:02X}{:02X}", r.round_rgb(), g.round_rgb(), b.round_rgb())
}

trait Round {
    fn round_rgb(&self) -> i32;
}

impl Round for i32 {
    fn round_rgb(&self) -> i32 {
        match self {
            i32::MIN..=-1 => 0,
            255..=i32::MAX => 255,
            _ => *self
        }
    }
}


macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
