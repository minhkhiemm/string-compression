fn main() {
    let input = "aaabbccaazzzdddiiiiiiijkllllllyyy";
    let input_array = to_array::<33>(input);
    let mut comparator = input_array[0];
    let mut result = String::new();
    let mut counter: u32 = 0;
    for v in input_array.iter() {
        if comparator == *v {
            counter += 1;
        } else {
            result.push(comparator);
            result.push(char::from_digit(counter, 10).unwrap());
            comparator = *v;
            counter = 0;
        }
    }
    println!("result: {}", result);
}

fn to_array<const N: usize>(s: &str) -> [char; N] {
    let mut chars = s.chars();
    [(); N].map(|_| chars.next().unwrap())
}
