use::std::env;

fn check(elem:&String)->f32{
    match elem.parse() {
        Ok(v) => v,
        Err(_) => std::process::exit(-1)
    }
}

fn add(args:&Vec<String>)->f32{
    let mut addition:f32 = 0.0;
    for i in 2..args.len(){
        addition += check(&args[i]);
    }
    return addition;
}

fn sub(args:&Vec<String>)->f32{
    let mut subtraction:f32 = check(&args[2]);
    for i in 3..args.len(){
        subtraction -= check(&args[i]);
    }
    return subtraction;
}

fn mul(args:&Vec<String>)->f32{
    let mut multiplication:f32 = 1.0;
    for i in 2..args.len(){
        multiplication *= check(&args[i]);
    }
    return multiplication;
}

fn div(args:&Vec<String>)->f32{
    let mut division:f32 = check(&args[2]);
        let mut option:Option<f32>;
        for i in 3..args.len(){
            if check(&args[i])==0.0{
                option = None;
            } else {
                option = Some(division/check(&args[i]));
            }
            match option {
                Some(d) => division = d,
                None => println!("Cannot divide by 0"),
            }
        }
        return division;
}

fn avg(args:&Vec<String>)->f32{
    let mut average: f32 = 0.0;
    let mut nr = 0.0;
    for i in 2..args.len(){
        average += check(&args[i]);
        nr += 1.0;
    }
    return average/nr;
}

fn sort(args:&mut Vec<String>)->Vec<&String>{
    let mut vSorted: Vec<&String> = Vec::new();
    let mut h = 0;
    args.sort();
    for i in 0..args.len()-2{ 
        vSorted.insert(h, &args[i]);
        h += 1;
    }
    return vSorted;
}

fn unique(args:&mut Vec<String>)->Vec<&String>{
    args.sort();
    let mut vUnique: Vec<&String> = Vec::new();
    let mut h = 0;
    for i in 0..args.len()-2 {
        if args[i] != args[i+1] {
            vUnique.insert(h, &args[i]);
            h += 1;
        }
    }
    return vUnique;
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args[1] == "add"{
        println!("Addition: {}", add(&args));
    } else if args[1] == "sub"{
        println!("Subtraction: {}", sub(&args));
    } else if args[1] == "mul"{
        println!("Multiplication: {}", mul(&args));
    } else if args[1] == "div"{
        println!("Division: {}", div(&args));
    } else if args[1] == "avg" {
        println!("Average: {}", avg(&args));
    } else if args[1] == "sort"{
        println!("Sorted array: {:?}", sort(&mut args));
    } else if args[1] == "unique"{ 
        println!("Array without dubloons: {:?}", unique(&mut args));
    }
}

#[cfg(test)]
mod tests;
