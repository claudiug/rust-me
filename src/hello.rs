fn main(){
    println!("hello claudiu");

    let x = 55i;
    //let (x, y) = (1i, 2i);i
    let a: int = 5;
    let mut xx = 5i;
    xx = 10i;
    println!("The value of xx is {}", xx);
    println!("The value of x is {}", x);
    println!("The value of a is {}", a);

    print_number(100);
    sum(100, 233);
    println!("value: {}", add(10,0));
}

fn print_number(x: int) {
    println!("Hmm, the number is: {}" , x);
}

fn sum(x: int, y: int) {
    println!("two numbers: {}, {}, and the sum: {}", x, y, x + y);
}

fn add(x: int, y: int) -> int {
    x + y
}

