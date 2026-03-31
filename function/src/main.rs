fn main() {
    println!("Hello, world!");

    my_function(); //function calling....
    another_function(70);
    print_labeled_measurement(5,'h');
    let y:bool=is_even(5);
    println!("the number is even: {y}");
}


fn my_function(){

    println!("Hello from my function")
}

fn another_function(x:i32){
    println!("This is value off x is {x}")
}

fn print_labeled_measurement(value:i32,unit_label:char){
    println!("The measurement is {value}{unit_label}")
}

fn is_even(x:i32) -> bool {
    if x%2 ==0{
        return true;
    }
    println!("the number is not even");
    return false
}