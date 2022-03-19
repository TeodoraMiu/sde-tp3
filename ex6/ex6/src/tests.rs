use crate::add;
use crate::sub;
use crate::mul;
use crate::div;
use crate::avg;
use crate::sort;
use crate::unique;

#[test]
fn addT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("add"), String::from("3"), String::from("4"), String::from("5"), String::from("2")];
    assert_eq!(add(&args), 14.0);
}

#[test]
fn subT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("sub"), String::from("12"), String::from("4"), String::from("5"), String::from("2")];
    assert_eq!(sub(&args), 1.0);
}

#[test]
fn mulT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("mul"), String::from("1"), String::from("4"), String::from("5")];
    assert_eq!(mul(&args), 20.0);
}

#[test]
fn divT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("div"), String::from("12"), String::from("4"), String::from("2")];
    assert_eq!(div(&args), 1.5);
}

#[test]
fn avgT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("avg"), String::from("10"), String::from("10"), String::from("7")];
    assert_eq!(avg(&args), 9.0);
}

#[test]
fn sortT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("sort"), String::from("1"), String::from("4"), String::from("2"), String::from("7"), String::from("0.5")];
    assert_eq!(sort(&mut args), vec![&String::from("0.5"), &String::from("1"), &String::from("2"), &String::from("4"), &String::from("7")]);
}

#[test]
fn uniqueT() {
    let mut args:Vec<String> = vec![String::from("file_path"), String::from("unique"), String::from("1"), String::from("7"), String::from("1"), String::from("3"), String::from("3"), String::from("8")];
    assert_eq!(unique(&mut args), vec![&String::from("1"), &String::from("3"), &String::from("7"), &String::from("8")]);
}