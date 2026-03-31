// const PI:u8=10; //can be global..


fn main() {
    //let fn =2; //cannot use fn as a variable name because fn is a keyword in Rust
    // let age:i32=24;
    // println!("value of age is {age}");
    // age=25; //cannot assign twice to immutable variable
    // println!("value of age is {age}");
    // const PI:u8=10;
    // println!("value of PI is {PI}");

    // const THREE_HOURS_IN_SECONDS:u32=60*60*3;
    // println!("value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    //i cannnot use age in this threehoursesceond because its let variable so how i can use? i can use by changeing let to const 

    // const AGE:u32=24;
    // println!("value of AGE is {AGE}");

    // const THREE_HOURS_IN_SECONDS:u32=60*60*3+AGE;
    // println!("value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // const ONE_HOUR:u64=60*60;

    // let time: u64=ONE_HOUR*30;

    // println!("value of time is {time}");

    // let apples : i32=10;

    // let apples : i32=20;         //showdoing....

    // println!("apples {apples}");


    // let apples: i32=10;

    // let apples : bool =true;

    // println!("apples {apples}");

    // let c: char='r';

    // let s : char='e';

   let emtiaz:(i32,bool,f64) = (56,true,100.00);

    let (x,y,z) =emtiaz;

   println!("x is {x}");
   println!("y is {y}");
   println!("z is {z}");
}
