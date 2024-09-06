use std::io;

fn main() {
    loop{println!("Enter any operation you want to perform.");
    println!("The operation you can use are => '+','-','*','/' or 'exit' for quitting the calculator");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("failed to read line");
    println!("Enter first number");
    let operator = operator.trim();
    if operator=="exit"{
        println!("quitting the calculator");
        break;
    }
    let mut first_num = String::new();
    io::stdin()
        .read_line(&mut first_num)
        .expect("failed to read line");
    
    let first_num: f32 = first_num.trim().parse().expect("the input wasn't number, Please enter a number");
    println!("Enter second number");
    let mut second_num = String::new();
    io::stdin()
        .read_line(&mut second_num)
        .expect("failed to read line");
    
    let second_num: f32 = second_num.trim().parse().expect("the input wasn't number, Please enter a number");
    if operator=="+"{
        let result=add(first_num,second_num);
        println!("The result is {}",result);
    }
    else if operator=="-"{
        let result=subtract(first_num, second_num);
        println!("The result is {}",result);
    }
    else if operator=="*"{
        let result=multiply(first_num, second_num);
        println!("The result is {}",result);
    }
    else if operator=="/"{
        if second_num!= 0.0 {let result=divide(first_num, second_num);
        println!("The result is {}",result);}
        else{
            println!("Error: Division by zero");
        }
        
    }

}}

fn add(x: f32, y: f32) -> f32 {
    x + y 
}


fn subtract(x: f32, y: f32) -> f32 {
    x - y  
}

fn multiply(x: f32, y:f32) -> f32{
    x*y
}
fn divide(x: f32,y:f32) -> f32{
    x/y
}

