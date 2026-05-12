// fn main() {
//     const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
//     println!("The value of THREE_HOURS_IN_SECONDS is : {}", THREE_HOURS_IN_SECONDS);
// }


// fn main(){
//     let x=5; //shadowing value 
//     let x=x+1;
//     {
//         let x=x*2;
//         println!("the value of x in the inner scope is:{}",x);
//     }
   
// }

// fn main(){
//     let spaces=" ";
//     let spaces=spaces.len();// shadowing with different type and get length of string
//     println!("the value of spaces is:{}",spaces);
// }

// scalar types ------------

// integers---> this is for interger and have two type signed and unsigned and have different size like 8,16,32,64 and 128 bit
            //    u32--> unsigned 32 bit integer + 
            //    i32--> signed 32 bit integer -
// floats --> this is for floating point numbers and have two type f32 and f64
// booleans --> this is for boolean values and have two value true and false
// characters--> this is for characters and have type char and can store a single character and use single quotes to define a char

// fn main(){
//   let x:f32=5.122323232; // this is for floating point numbers and have two type f32 and f64
//   println!("the value of x is:{}",x);
// }

// fn main(){
//   let x:char='A'; // this is for characters and have type char and can store a single character and use single quotes to define a char
//   println!("the value of x is:{}",x);
// }

// fn main(){
//   let x:bool=true; // this is for boolean values and have two value true and false
//   println!("the value of x is:{}",x);
// }



// compound types ------------

//Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

// tuples --> this is for grouping multiple values of different types and have a fixed length and use parentheses to define a tuple
// arrays --> this is for grouping multiple values of the same type and have a fixed length and use square brackets to define an array

// fn main(){
//     let tupe:(i32,f32,bool,f32)=(500,6.4,true,7.8); // this is for grouping multiple values of different types and have a fixed length and use parentheses to define a tuple
//     let a=tupe.0; // this is for accessing the value of a tuple and use dot notation to access the value of a tuple
//     println!("the value of a is: {}",a);
//     println!("the value of tupe is: {:?}",tupe);
// }

fn main(){
    let arr:[i32;5]=[1,2,3,4,5]; // this is for grouping multiple values of the same type and have a fixed length and use square brackets to define an array
    let a=arr[0];
    println!("the value of a is: {}",a);
    println!("the value of arr is: {:?}",arr);
    another_function(); // this is for calling a function and use the name of the function to call a function
    back_success_state(12);
    loop_test();
    loop_array();
}

// functions fn --> this is for defining a function and use fn keyword to define a function and use parentheses to define the parameters of a function and use curly braces to define the body of a function
fn another_function(){
    println!("this is another function");
  let sum_number=  sum(2,10);
  println!("the value of sum is: {}",sum_number);
}


fn sum(a:i32,b:i32 )->i32{ // this is for defining a function that takes two parameters and returns an integer and use the return type of a function to define the return type of a function
    a+b // this is for returning the value of a function and use the return keyword to return the value of a function
}

fn back_success_state(result:i32)->bool{
  if result>10{
    println!("the result is greater than 10");
    return true;
  }
  else{
    println!("the result is less than or equal to 10");
    return false;
  } 
}


fn loop_test(){
  let mut count =5 ;
  loop{
    println!("the value of count is: {}",count);
    count-=1;
    if count==0{
      break;
    }
  }
}

fn loop_array(){
    let arr:[i32;5]=[1,2,3,4,5];
    for element in arr.iter(){ // this is for iterating over an array and use the iter method to iterate over an array
        println!("the value of element is: {}",element);
    }
}

