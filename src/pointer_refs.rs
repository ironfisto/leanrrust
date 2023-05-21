pub fn run() {

    // array permitive data
    let y = [1,2,3];
    let x = y;
    // should get no error
    println!("{:?}",(y,x));

    // vec non permitive data
    let yy = vec![1,2,3];
    let xx = &yy;
    // should get no error
    println!("{:?}",(&yy,xx))

}