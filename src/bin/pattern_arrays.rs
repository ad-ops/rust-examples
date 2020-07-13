use std::env;

/*
 Try args:
    x
    y
    y y
    x y z
    [empty]
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let str_vec: Vec<&str> = args.iter().map(|x| x.as_str()).collect();

    let result = match str_vec.as_slice() {
        [s] => format!("Only one and it was: {}", s),
        [_, r, s, _] => format!(
            "Didn't care about first or last item, but the middle ones were: {}, {}",
            r, s
        ),
        [.., r, s] if r == s => format!("The last two where the same! {} == {}", r, s),
        [.., "x"] => "Last item was x".into(),
        [a @ .., s] => format!("Last item was not x, but {}, and the head is {:?}", s, a),
        _ => "No rule".into(), // will never occur since args always contains path to bin.
    };
    println!("{:?}: {}", args, result);
}
