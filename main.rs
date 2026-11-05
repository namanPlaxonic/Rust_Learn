fn main(){
    println!("Hi Naman!");
    let mut  x: i8=99;
    println!("The value of x is: {}", x);
    let z=true;
    print!("The value of z is: {}", z);

    print!("\n");

    let ismale :bool= false;
    let is_over_18: bool= true;

    if ismale{
        println!("The person is male.");
    } 
    else {
        println!("The person is not a male.");
    }
    

    if is_over_18 && ismale{
        println!("The person is legal male.");
    }
    
x=22;
println!("The value of x is: {}", x);

// strings in Rust

let name : &str= "Naman";
println!("The name is: {}", name);

let x:u32=0;

for i in 0..270{
    print!("\n  {}", x+i);
} 

let greetings: String= String::from("Hello, Naman!");
println!("\n{}", greetings);


let result = do_sum(56, 14);
println!("The sum is: {}", result);     

}

fn do_sum(a:i32, b:i32)->i32{
    return a+b;
}