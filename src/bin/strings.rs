fn main() {
    let hello = "Hello World!";
    println!("{}", hello);
    println!("{}", hello.to_string());

    hello
        .chars()
        .enumerate()
        .for_each(|(i, c)|
            println!("{}:{}", i, c)
        );

    let rev = hello
        .chars()
        .into_iter()
        .rev()
        .collect::<String>();
    println!("{} <-> {}", rev, hello);

    let mut vec = hello
        .chars()
        .into_iter()
        .collect::<Vec<char>>();
    vec.sort();
    let sorted = vec
        .into_iter()
        .collect::<String>();
    print!("Sorted: {}", sorted);

    println!("Chunked:");
    hello
        .chars()
        .into_iter()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .for_each(|part| println!("{}", part));

    let joined = hello
        .chars()
        .map(|c| c.to_string())
        .into_iter()
        .collect::<Vec<String>>()
        .join(".");
    println!("Joined: {}", joined);
}