pub fn run() {
 greet("Mukul");
    let sum = add(1,2);
    println!("{}",sum);

    // clouser functions
    let add_nums = |n:i8,n2:i8| n + n2 ;
    println!("{}",add_nums(1,2))
}

fn greet(name : &str) {
    println!("{}",name)
}
// return functions

fn add(v:i8,c :i8) -> i8{
    v +c
}