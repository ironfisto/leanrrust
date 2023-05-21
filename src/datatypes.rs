use std::i64::MAX;

pub fn run() {
    // int and uint
    let x = 1;
    let y: u32 = 2;

    println!("{:?}", MAX);
    // boolean
    let hello: bool = true == false;
    println!("{:?}", hello);

    //char -> in single quotes , only one chars not more then one
    let xchar = 'a';
    let ychar = '\u{1F600}';
    println!("{:?}",ychar)

}