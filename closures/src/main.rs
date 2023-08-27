// fn main() {
//     let square_closure = |num: &String| println!("String is {}", num);
//     let num = String::from("Bala");
//     square_closure(&num);
//     print!("after calling the closure num is {}", num);
// }
// fn can_be_divided<F: Fn(f32) -> bool>(num1: f32, num2: f32, function: F) {
//     if function(num2) {
//         println!("can not be divided");
//     } else {
//         println!("Can be divided");
//     }
// }
// fn main() {
//     let division_status = |num: f32| {
//         if num == 0.0 {
//             true
//         } else {
//             false
//         }
//     };
//     println!("{}", division_status(0.0));
//     println!("{}", division_status(1.0));
//     can_be_divided(3.0, 0.0, division_status);
// }
fn main() {
    //Types to define the closures
    let s1 = |x: u32| x + 1;
    let s2 = |x| x + 2;

    s2(290);
    println!("{}", s1(300));
}
