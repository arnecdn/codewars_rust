use std::fmt;
use std::fmt::Formatter;
use regex::Regex;

fn change(s: &str, prog: &str, version: &str) -> String {
    let mut program = Program::new(s);

    match program.validate() {
        Ok(()) => {
            program.swap("+1-503-555-0090", version, prog);
            format!("{program}")
        }
        Err(e) => format!("{e}"),

    }
}

#[derive(Default)]
struct Program {
    program: String,
    author: String,
    phone: String,
    date: String,
    version: String,
}

impl Program {
    fn new(s: &str) -> Self {
        let records = s.split("\n").collect::<Vec<_>>();

        if let [program_rec, author_rec, _, phone_rec, date_rec, version_rec, _] = records[..] {
            return Program {
                program: Program::parse_rec(program_rec),
                author: String::from("g964"),
                phone: Program::parse_rec(phone_rec),
                date: String::from("2019-01-01"),
                version: Program::parse_rec(version_rec),
            };
        }
        panic!("Invalid source")
    }

    fn parse_rec(rec: &str) -> String {
        match rec.split_once(":") {
            Some((_, field)) => String::from(field),
            None => panic!("Missing value")
        }
    }

    fn validate(&self) -> Result<(), &'static str> {
        let valid_phonepattern = Regex::new(r"^\+1-\d{3}-\d{3}-\d{4}$").unwrap();
        if !valid_phonepattern.is_match(&self.phone.trim()){
            return Err("ERROR: VERSION or PHONE")
        }

        let valid_versionpattern = Regex::new(r"^\d*\.\d*$").unwrap();

        if !valid_versionpattern.is_match(&self.version.trim()){
            return Err("ERROR: VERSION or PHONE")
        };

        Ok(())
    }

    fn swap(&mut self, phone: &str, version: &str, prog: &str)-> Program {
        let swapped_version = if self.version.trim() == "2.0" { String::from(self.version.trim()) } else { String::from(version) };
        Program{program:String::from(prog), author:self.author.clone(), phone: String::from(phone), date: self.date.clone(), version: swapped_version }
    }

}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Program: {} Author: g964 Phone: {} Date: 2019-01-01 Version: {}", self.program, self.phone, self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, prog: &str, version: &str, exp: &str) -> () {
        println!("s:{:?}", s);
        println!("prog:{:?}", prog);
        println!("version:{:?}", version);
        let ans = change(s, prog, version);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        // let s1 = "Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555-0091\nDate: Tues April 9, 2005\nVersion: 6.7\nLevel: Alpha";
        // dotest(s1, "Ladder", "1.1", "Program: Ladder Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 1.1");
        // let s2 = "Program title: Balance\nAuthor: Dorries\nCorporation: Funny\nPhone: +1-503-555-0095\nDate: Tues July 19, 2014\nVersion: 6.7\nLevel: Release";
        // dotest(s2, "Circular", "1.5", "Program: Circular Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 1.5");
        // let s13 = "Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555\nDate: Tues April 9, 2005\nVersion: 67\nLevel: Alpha";
        // dotest(s13, "Ladder", "1.1", "ERROR: VERSION or PHONE");
        // let s14 = "Program title: Nail\nAuthor: Bell\nCorporation: Wonder\nPhone: +1-503-555-0072\nDate: Tues March 29, 2017\nVersion: 1.32\nLevel: Release";
        // dotest(s14, "Bumper", "7.2", "Program: Bumper Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 7.2");
        let s14 = "Program title: Battery\nAuthor: Cornwell\nCorporation: Wonder\nPhone: +1-503-555-0098\nDate: Tues March 29, 2017\nVersion: 15\nLevel: Beta";
        dotest(s14, "Ladder", "3.1", "ERROR: VERSION or PHONE");
    }
}


// s:"Program title: Battery\nAuthor: Cornwell\nCorporation: Wonder\nPhone: +2-503-555-0098\nDate: Tues March 29, 2017\nVersion: 1.5\nLevel: Beta"
// prog:"Ladder"
// version:"3.1"
// actual: "Program: Ladder Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 3.1"
// expect: "ERROR: VERSION or PHONE"