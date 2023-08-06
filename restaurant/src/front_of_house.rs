pub mod hosting;

mod serving {
    fn take_order() {}
    fn server_order() {}
    fn take_payment() {}
}
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
