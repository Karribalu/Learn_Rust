mod person{
    ///----------------------------------
    /// This is for multiplication
    /// ```
    /// let n = 4
    /// assert_eq!(4,n);
    /// ```
    //! Some function
    /// ---------------------------------
    pub struct person_info{
        pub name: String,
        pub age: i32
    }
    impl person_info{
        pub fn new(name: &str, age: &i32) -> Self{
            Self{
                name: String::from(name),
                age: age.clone()
            }
        }
    }
}

pub fn create_person(){
    let mut new_person = person::person_info::new(&"bala", &23);
    new_person.name = String::from("shiva");

    let mut aaa = person::person_info{
        age: 20,
        name: String::from("some")
    };
}