pub fn generics_1() {
    let numbers = vec![30, 50, 70, 10];
    let numbers2 = vec![50, 100, 80, 30];
    println!("the largest number is {}", largest(&numbers));
    println!("the largest number2 is {}", largest(&numbers2));

    fn largest(numbers: &[i32]) -> &i32 {
        let mut largest_number = &numbers[0];
        for number in numbers {
            if number > largest_number {
                largest_number = number;
            }
        }
        largest_number
    }
}

pub fn generics_2() {
    let numbers = vec![30, 50, 70, 10];
    let list2 = vec!['b', 'f', 'r', 'h'];
    println!("the largest number is {}", largest(&numbers));
    println!("the largest character is {}", largest(&list2));

    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
}
