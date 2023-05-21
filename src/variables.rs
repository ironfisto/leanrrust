pub fn run() {

    // we can not chnage value
/*    let hello = "String";
    hello = "Change it";*/

    // we cacn change value
    let mut hello = "String";
    hello = "Change it";
    println!("{val}", val = hello);

    // const
    const ID :&str = "ee";

    // multiple vars
    let (x,y) = (10, 20);
    println!("{:?}",(x,y))

}