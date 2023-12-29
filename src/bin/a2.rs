// your mission is to write a rust function for add two number
// and print the sum up how ever you will need two function for this
// one called add_numbers the other called display_output
// you may not use one function for this or two function or wise mission
// fail



/// function to add two numbers
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
///
/// The sum of `a` and `b`
fn add_numbers(a: i32, b: i32) -> i32{
    a + b
}

/// functon to day the result of a operation
///
/// # Arguments
///
/// * `result` - the rust to output
/// # Return
///
/// void (nothing)
fn display_output(result:i32){
    println!("{:?}", result);
}


fn main(){
    let number_sum = add_numbers(3, 4);
    display_output(number_sum);
}
