pub fn run() {
    //https://youtu.be/zF34dRivLOw?t=3163
    // arrays are stack allocated

    // can store fixed data types
    let num = [1, 2, 3, 4, 5];
    let strr = ["aaa"];
    println!("{:?}", num);
    println!("{:?}", strr);

    // get sngle value
    println!("{:?}", strr.get(0));
    println!("{:?}", strr[0]);

    // can reassign vale
    let mut chan = [1, 3];
    chan[0] = 2;

    // get stack
    // print!(size_of_val(&chan));
}