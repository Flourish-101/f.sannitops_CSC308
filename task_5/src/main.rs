fn main() {
    let s1 = String::from("Hi");
    let s2 = String::from("amazing!");

    let result = longest(&s1,&s2);
    println!("The longer string is {}", result);

}

fn longest(a:&String, b:&String) ->String{
    if a.len()>b.len(){
        a.to_string()
    }else{
        b.to_string()
    }
}
