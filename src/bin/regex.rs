use regex::Regex;

fn main() {
    let date_pattern = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    let text = "hello this text is written 2020-05-27 and I will go on vacation on 2020-06-08 and back on 2020-06-29".to_string();
    let mat_iter = date_pattern.find_iter(&text);
    
    for mat in mat_iter {
        println!("Found match between {}-{} which is '{}'", mat.start(), mat.end(), mat.as_str());
    }
}