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
    let choice; //$Env:choice="add";
    match env::var("choice"){
        Ok(val) => choice = val,
        Err(_e) => choice = "none".to_string(),
    }
    let number1; //$Env:choice=1;
    let number2; //$Env:choice=2;
    match env::var("number1"){
        Ok(val) => number1 = val,
        Err(_e) => number1 = "none".to_string(),
    }
    match env::var("number2"){
        Ok(val) => number2 = val,
        Err(_e) => number2 = "none".to_string(),
    }
    if choice=="add"{
        println!("Result of the addition is {}", add(check(number1.clone()), check(number2.clone())));
    } else if choice=="sub"{
        println!("Result of the subtraction is {}", sub(check(number1.clone()), check(number2.clone())));
    } else if choice=="div"{
        let res2 = div(check(number1.clone()),check(number2.clone()));
        match res2 {
            Some(r) => println!("The result of the division is {}",r),
            None => println!("Cannot divide by 0"),
        }
    } else if choice=="mul"{
        println!("Result of the multiplication is {}", mul(check(number1.clone()), check(number2.clone())));
    } else if choice=="rem"{
        let res3 = rem(check(number1.clone()),check(number2.clone()));
        match res3 {
            Some(r) => println!("The remainder of the division is {}",r),
            None => println!("Cannot divide by 0"),
        }
    } else {println!("Choose between add, sub, div, mul, rem.")};

}
