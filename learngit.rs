fn main() {
    println!("Size of a char is {}", std::mem::size_of::<char>()); // 4 byte
    println!("Size of a: {}", "a".len());  // 1 byte
    /* WOW */
    let num = {
        let num2 = 9;
        num2 + 9
    };
    let num3 = num + num;
    println!("Sum of {num} and {num} is {num3:?}");

    let nulus = ();
    println!("Woopss {:?}", nulus); // {:#?} for pretty-print

    print!("Max of usize is {}", usize::MAX);
    println!(" and Min is {}", usize::MIN);
    print!("Max of isize is {}", isize::MAX);
    println!(" and Min is {}", isize::MIN);
}