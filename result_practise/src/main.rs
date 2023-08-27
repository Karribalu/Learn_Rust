fn division(divident: f64, divisor: f64) -> Result<f64, String>{
    if(divisor == 0.0){
        Err(String::from("Denominator is 0"))
    }
    else{
        Ok(divident / divisor)
    }
}

fn main() {
    println!("{:?}", division(5.0, 2.0).unwrap());
    println!("{:?}", division(6.0,0.0).unwrap_or(0.0));
}
