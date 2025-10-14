use std::io;
fn main() {
    println!("Assignment");

    println!("Customer, Enter your energy consumption usage(KWh): ");
    let mut usage = String::new();
    
    io::stdin()
       .read_line(&mut usage)
       .expect("Failed to read line");
    
    let usage:u32 = usage.trim().parse().unwrap_or(0);

    let rate = if usage>200{
        30
    }else if usage>100{
        25
      }else{
        20
    } ; 

    println!("Rate : #{}", rate);
    let bill = rate*usage;
    println!("Total electricity bill: #{}", bill);
}
