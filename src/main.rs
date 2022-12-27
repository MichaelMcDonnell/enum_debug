#[derive(Debug)]
enum Simple {
    A,
    B,
    C,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Advanced {
    A(u32),
    B { value: u32 },
    C,
}

fn main() {
    let simple_a = Simple::A;
    let simple_b = Simple::B;
    let simple_c = Simple::C;
    let advanced_a = Advanced::A(123);
    let advanced_b = Advanced::B { value: 456 };
    let advanced_c = Advanced::C;

    println!("simple_a = {:?}", simple_a);
    println!("simple_b = {:?}", simple_b);
    println!("simple_c = {:?}", simple_c);
    println!("advanced_a = {:?}", advanced_a);
    println!("advanced_b = {:?}", advanced_b);
    println!("advanced_c = {:?}", advanced_c);
    println!("Done!");
}
