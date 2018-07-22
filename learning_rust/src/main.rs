fn main() {
    let mut y;
    {
        let x = 1;

        y = &x;
    }
    println!("{}", y);
}