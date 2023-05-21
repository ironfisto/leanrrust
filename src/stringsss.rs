pub fn run() {
    // this is fixed length and immutable stores in memory
    let x = "aa";

    // heap allocated string and growable
    let y = String::from("Helllo");
    println!("{:?}", y);
    // get lenght
    println!("{:?}", y.len());
    // push to string
    let mut z = String::from("aaa");
    // only pushes one char
    z.push('l');
    // push stringh
    z.push_str("aaaaa");
    println!("{:?}", z);
    // check if empty
    z.is_empty();
    // loop through string : TODO
    // initiate string with capacity : TODO

}