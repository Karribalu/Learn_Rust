// #[cfg(test)]
// mod tests{
//     #[test]
//     fn it_works(){
//         assert_eq!(123,123);
//     }
//
//     #[test]
//     fn it_not_works(){
//         assert_eq!(123,12);
//     }
// }

struct Circle{
    radius: f32
}
impl Circle{
    fn area(&self) -> f32{
        3.14 * (&self.radius * &self.radius)
    }
    fn perimeter(&self) -> f32{
        2.0 * 3.14 * &self.radius
    }

    fn contains(&self, other: &Circle) -> bool{
        self.radius > other.radius
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn area_test(){
        let circle = Circle{ radius: 5.0};
        assert_eq!(78.5, circle.area());
        let smaller_circle = Circle{radius: 3.9};
        circle.contains(&smaller_circle);
    }

    #[test]
    fn contains_test(){
        let circle = Circle{ radius: 5.0};
        let smaller_circle = Circle{radius: 3.9};
        circle.contains(&smaller_circle);
    }
}