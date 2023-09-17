pub mod basic {
    pub trait personal_info {
        fn info() -> Self;
    }
    impl person_info for personal_info {
        fn info() -> Self{
            return "hello";
        }
    }
}

fn get_person(){
    use basic::
}
