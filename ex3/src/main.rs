use::std::env;

fn div(arg1:f32, arg2:f32)->Option<f32>{
    if arg2==0.0{
        return None;
    }
    return Some(arg1/arg2);
}

fn add(arg1:f32, arg2:f32)->f32{
    return arg1+arg2;
}

fn sub(arg1:f32, arg2:f32)->f32{
    return arg1-arg2;
}

fn mul(arg1:f32, arg2:f32)->f32{
    return arg1*arg2;
}

fn rem(arg1:f32, arg2:f32)->Option<f32>{
    if arg2==0.0{
        return None;
    }
    return Some(arg1%arg2);
}

fn check(elem:String)->f32{
    match elem.parse() {
        Ok(v) => v,
        Err(_) => std::process::exit(-1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1]=="add"{
        println!("Result of the addition is {}", add(check(args[2].clone()), check(args[3].clone())));
    } else if args[1]=="sub"{
        println!("Result of the subtraction is {}", sub(check(args[2].clone()), check(args[3].clone())));
    } else if args[1]=="div"{
        let res2 = div(check(args[2].clone()),check(args[3].clone()));
        match res2 {
            Some(r) => println!("The result of the division is {}",r),
            None => println!("Cannot divide by 0"),
        }
    } else if args[1]=="mul"{
        println!("Result of the multiplication is {}", mul(check(args[2].clone()), check(args[3].clone())));
    } else if args[1]=="rem"{
        let res3 = rem(check(args[2].clone()),check(args[3].clone()));
        match res3 {
            Some(r) => println!("The remainder of the division is {}",r),
            None => println!("Cannot divide by 0"),
        }
    } else {println!("Choose between add, sub, div, mul, rem.")};
}
