//Made by me to stick it to a friend
use std::{
    io,//array
};

fn main(){
    let active = true;
    tutorial();

    while active{
        let mut color_string=String::new(); 
        let color_vector:Vec<&str>;

        second_try();

        //input
        io::stdin()
            .read_line(&mut color_string)
            .expect("Failed to read line"); 

        color_vector = make_vector(&color_string); 

        let size =color_vector.len();//get the length for the match
        match size {

            4..7 => resistor_math(color_vector,size),
            _=>println!("\nResistors can only be 4 to 6 colors long. Please try again.\n")       
        }
        
        continue;
    }//while active
}

///Second message.
/// reappears each loop cycle for the resistor syntax
fn second_try(){
    println!("[BL]ack, [B]rown, [b]lue, [r]ed, [o]range, [y]ellow, [GR]ay, [G]old, [g]reen, [s]ilver, [w]hite, [v]iolet");  
}

///Tutorial message
/// 
/// plays the  welcome message, prompting users to type colors.
/// Only plays one time.
fn tutorial() {
    println!("Welcome to Trisistors. \n
    This program handles all normal resistors with 4 to 6 bands\n
    INPUT COLORS LIKE EXAMPLE: g b r G == green blue red Gold\n");
}

///Matches the color to its value
///  * `color` - The color code slice being evaluated.
fn number_match(color: &str ) -> &str{
    match color{
    
        "BL" => "0",
        "B" => "1",
        "r" => "2",
        "o" => "3",
        "y" => "4",
        "g" => "5",
        "b" => "6",
        "v" => "7",
        "GR" => "8",
        "w" => "9",
        _ => "error!", // This catches anything that isn't listed above. change it a little bit if the thing doesn't work right.
    }

}

///Turn the resistor color string into a Vector
/// 
/// * `resistor_string` - the string of the resistor colors
/// * `colors` - The completed vector of the resistor colors
fn make_vector(resistor_string: &str) -> Vec<&str>{

    let colors: Vec<&str> = resistor_string.split_whitespace().collect();
    return colors;
}

///Flips the Vector and returns the flipped vector
fn flip_vector(mut color_vector: Vec<&str>) -> Vec<&str>{

    color_vector.reverse();

    color_vector
}

/// Turn the multiplying color into its value
/// 
///  * `color` - the multiplication slice
fn multiple_match(color: &str) -> &str{
    match color{
        "BL" => "Ω",
        "B" => "0Ω",
        "r" => "00Ω",
        "o" => " X 1kΩ",
        "y" => " X 10kΩ",
        "g" => " X 100kΩ",
        "b" => " X 1MΩ",
        "G" => " X 0.1Ω",
        "s" => " X 0.01Ω",
        _=> "error!",
    }
}

///Turn the color slice to its tolerance value
fn tolerance_match(color: &str) ->&str{
    match color{
        "B" => "1",
        "r" => "2",
        "g" => "0.5",
        "b" => "0.25",
        "v" => "0.1",
        "G" => "5",
        "s" => "10",
        _=> "error!",
        
    }
}

///Turn the color slice to the resistor's temperature coefficient
fn temp_match(color:&str) ->&str{
    match color{
       "BL" => "250",
        "B" => "100",
        "r" => "50",
        "o" => "15",
        "y" => "25",
        "g" => "20",
        "b" => "10",
        "v" => "5",
        "GR" => "1",
        _ => "error!",
    }

}

///calculates and prints the value of the resistor.
/// 
/// * `color_vector` - The vector of the color bands on the resistor.
/// 
/// * `bands` - The total number of color bands on the resistor. Thought of as size, derived from color_vector.
fn resistor_math(mut color_vector: Vec<&str>,bands:usize){
    let mut reversi = String::new();
    let mut active = true;

    while active{
        let mut value_vector : Vec<&str>=Vec::new();
        let mut i = 0;
        reversi.clear();

            //let this happen regardless, always has at least 2 number bands
        while i < 2 {//handles digit 1 and 2

            let result = number_match(color_vector[i]);
            value_vector.push(result);

            i+=1;
        }
        
        //this is what coding is all about. flexibility and ultimate performance
    match bands {
        4 => {
            value_vector.push(multiple_match(color_vector[i]));

            value_vector.push(tolerance_match(color_vector[i+1]));

            println!("\nResistor Value: {}{}{} +/- {}%",value_vector[0],value_vector[1],value_vector[2],value_vector[3]);
        }
        5 =>{
            value_vector.push(number_match(color_vector[i]));

            value_vector.push(multiple_match(color_vector[i+1])); 

            value_vector.push(tolerance_match(color_vector[i+2]));

            println!("\nResistor Value: {}{}{}{} +/- {}%",value_vector[0],value_vector[1],value_vector[2],value_vector[3],value_vector[4]);
        }
        6 =>{
            value_vector.push(number_match(color_vector[i]));

            value_vector.push(multiple_match(color_vector[i+1])); 

            value_vector.push(tolerance_match(color_vector[i+2]));

            value_vector.push(temp_match(color_vector[i+2]));

            println!("\nResistor Value: {}{}{}{} +/- {}% , {} ppm/K",value_vector[0],value_vector[1],value_vector[2],value_vector[3],value_vector[4],value_vector[5]);
        }
        _ => println!("something went very very wrong."),
    }
    //only one reversi function needed.
        println!("Reverse colors to fix errors? \n Y/N?");

        io::stdin()
                .read_line(&mut reversi)
                .expect("Failed to read line");

        if reversi.trim().eq_ignore_ascii_case("Y"){
            color_vector = flip_vector(color_vector);
            
        }else{
            active = false;
        }
}
}
//saved 56 lines of code.