fn main() {
    let v = vec![1, 3, 4];
    println!("{:?}", v);

    let mut v_new = Vec::new();
    v_new.push(10);
    v_new.push(20);
    println!("{:?}", v_new);

    let second = &v[1];
    println!("{}", second);
    println!("{:?}", v_new);
    // v_new.push(20);
    for i in &mut v_new {
        *i += 50;
    }
    println!("{:?}", v_new);
}
