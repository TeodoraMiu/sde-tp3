fn div(arg1:f32, arg2:f32)->Option<f32>{
    if arg2==0.0{
        return None;
    }
    return Some(arg1/arg2);
}

fn main() {
    let res = div(3.0,0.0);
    match res {
        Some(r) => println!("The result of the division is {}.", r),
        None => println!("Cannot divide by 0."),
    }
}
