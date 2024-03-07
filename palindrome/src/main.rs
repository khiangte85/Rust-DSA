fn palindrome<T: PartialEq>(input: &[T]) -> bool {
    let len = input.len();

    for i in 0..len / 2 {
        if input[i] != input[len - i - 1] {
            return false;
        }
    }

    true
}

fn main() {
    let a = "racecar";
    let b = [1,2,3,3,4,23];
    let c = String::from("hello world");
    let d = vec![6,7,8,9,0,9,8,7,6];

    println!("{} is palindrome: {:?}", a, palindrome(&a.chars().collect::<Vec<_>>()));
    println!("{:?} is palindrome: {:?}", b, palindrome(&b));
    println!("{:?} is palindrome: {:?}", c, palindrome(&c.chars().collect::<Vec<_>>()));
    println!("{:?} is palindrome: {:?}", d, palindrome(&d));
}
