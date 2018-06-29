fn main() {
    for num in (0..1000) {
        match [num % 3, num % 5, num] {
            [0, 0, _] =>  println!("FizzBuzz"),
            [_, 0, _] =>  println!("Buzz"),
            [0, _, _] =>  println!("Fizz"),
            _ =>  println!("{}", num),
        }
    }
}
