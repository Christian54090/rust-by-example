fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // you can right align text with a specified width. this will put in 5 white spaces
    println!("{number:>width$}", number=1, width=6);

    // you can pad numbers with extra zeroes. this will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Activities
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}