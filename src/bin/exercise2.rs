
    struct Circle{
        radius:f64,
    }

    impl Circle{
        fn area(&self)->f64{
            self.radius*self.radius*3.14
        }
        fn circumference(&self)->f64{
            2.0*3.14*self.radius
        }
    }

fn main(){
    let c = Circle{
        radius: 3.0
    };
    println!("Area of circle is: {}", c.area());
    println!("Circumference of circle is: {}", c.circumference());
}