use::std::env;

fn div(arg1:f32, arg2:f32)->Option<f32>{
    if arg2==0.0{
        return None;
    }
    return Some(arg1/arg2);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = match args[1].parse() {
        Ok(v) => v,
        Err(_) => std::process::exit(-1)
    };

    let b = match args[2].parse() {
        Ok(v) => v,
        Err(_) => std::process::exit(-1)
    };

    let res1 = div(a,b);
    match res1 {
        Some(r) => println!("The result of the division is {}", r),
        None => println!("Cannot divide by 0."),
    }
}
