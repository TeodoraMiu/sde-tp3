use std::collections::HashMap;
use::std::env;

fn check(elem:&String)->f32{
    match elem.parse() {
        Ok(v) => v,
        Err(_) => std::process::exit(-1)
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args[1] == "add"{
        let mut addition:f32 = 0.0;
        for i in 2..args.len(){
            addition += check(&args[i]);
        }
        println!("Addition: {}", addition);
    } else if args[1] == "sub"{
        let mut subtraction:f32 = check(&args[2]);
        for i in 3..args.len(){
            subtraction -= check(&args[i]);
        }
        println!("Subtraction: {}", subtraction);
    } else if args[1] == "mul"{
        let mut multiplication:f32 = 1.0;
        for i in 2..args.len(){
            multiplication *= check(&args[i]);
        }
        println!("Multiplication: {}", multiplication);
    } else if args[1] == "div"{
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
        println!("Division: {}", division);
    } else if args[1] == "avg" {
        let mut average: f32 = 0.0;
        let mut nr = 0.0;
        for i in 2..args.len(){
            average += check(&args[i]);
            nr += 1.0;
        }
        println!("Average: {}", average/nr);
    } else if args[1] == "sort"{
        args.sort();
        for i in 0..args.len()-2{ //e cam urat asa dar nu prea stiu cum sa scot valorile care nu sunt numere, ca daca folosesc check imi da eroare si se opreste programul
            print!("{} ", check(&args[i]));
        }
    } else if args[1] == "unique"{ //am presupus ca inseamna sa fie scoase dublurile
        args.sort();
        let mut vUnique: Vec<&String> = Vec::new();
        let mut h = 0;
        for i in 0..args.len()-2 {
            if args[i] != args[i+1] {
                vUnique.insert(h, &args[i]);
                h += 1;
            }
        }
        println!("{:?}",vUnique);
    }
}
