fn main() {
    println!("Classwork 2\n");
    let price:f64 = 5000.0;
    let d1:f64 = 0.1*price;
    let d2:f64 = 0.15*price;
    let d3:f64 = 0.0;

    if price>=10000.0{
       let bill = price-d2;
       println!("price ={}", price);
       println!("discount = {}", d2);
       println!("bill = {}", bill);
    }else if price>=5000.0{
       let bill = price-d1;
       println!("price ={}", price);
       println!("discount = {}", d1);
       println!("bill = {}", bill);
    }else{
        let bill = price;
        println!("price ={}", price);
        println!("discount = {}",d3);
        println!("bill = {}", bill);
    }
}
