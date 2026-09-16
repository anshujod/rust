fn main() {
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &s; // BIG PROBLEM

    println!("{r1}, {r2}, {r3}");
}