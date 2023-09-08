fn main() {
    let some_vector = vec![1, 2, 3, 5, 6, 7];
    let mut iterator = some_vector.iter();
    println!("iterator is {:?}", iterator);
    // let check = &iterator.find(|&&x| x == 6);
    // println!("{:?}", check);

    let filter = iterator.filter(|&x| *x > 3).collect::<Vec<&u32>>();
    println!("{:?}", filter);

    let square = some_vector
        .iter()
        .map(|x| *x * *x)
        .filter(|&x| x > 10)
        .collect::<Vec<u32>>();
    print!("{:?}", square);
}
