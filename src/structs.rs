// tuple struct
struct Tcolor(u8);

// traditional struct
struct Color {
    r :u8
}

pub fn run() {
   let c : Color = Color{r:22};
    println!("{:?}",c.r);
    let cc = Tcolor(1);
    println!("{:?}",cc.0);
}

