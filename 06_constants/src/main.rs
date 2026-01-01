const DISCOUNT_RATE: f32 = 0.1;

fn main() {
    let toy_price: f32 = 29.99;
    let discounted_price: f32 = toy_price - (toy_price * DISCOUNT_RATE);
    println!("The discounted price of the toy is: ${:.2}", discounted_price);
}
