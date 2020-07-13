fn main() {
    
    let special_rule_active = true;
    for i in 1..=10 {
        let simple = match i {
            1 => "it was the first",
            2 | 3 => "2 or 3",
            4..=5 if special_rule_active => "4 to 5 special",
            v @ 6..=8 if v == 7 => "6 to 8 but only 7",
            v if v > 9 => "larger than 9",
            _ => "no rule"
        };
        println!("{}: {}", i, simple);
    }
    
}