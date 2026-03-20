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
            4 => four_band_resistor(color_vector),
            5 => five_band_resistor(color_vector),
            6 => six_band_resistor(color_vector),
            _ => println!("resistor bands are always 4-6. please try again."),
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
    INPUT COLORS LIKE EXAMPLE: g b r G \n== green blue red Gold\n");
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

///The Four band Resistor Function
fn four_band_resistor(mut color_vector: Vec<&str>){

    let mut reversi = String::new();
    let mut active = true;

    while active{

        let mut value_vector : Vec<&str>=Vec::new();
        let mut i = 0;
        reversi.clear();

        while i < 2 {//handles digit 1 and 2

            let result = number_match(color_vector[i]);
            value_vector.push(result);

            i+=1;
        }

        value_vector.push(multiple_match(color_vector[i])); //save the value into the vector

        value_vector.push(tolerance_match(color_vector[i+1]));

        println!("\nResistor Value: {}{}{} +/- {}%",value_vector[0],value_vector[1],value_vector[2],value_vector[3]);
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

///The Five band Resistor Function
fn five_band_resistor(mut color_vector: Vec<&str>){

    let mut reversi = String::new();
    let mut active = true;

    while active{

        let mut value_vector : Vec<&str>=Vec::new();
        let mut i = 0;
        reversi.clear();

        while i < 3 {//handles digit 1 and 2

            let result = number_match(color_vector[i]);
            value_vector.push(result);

            i+=1;
        }

        value_vector.push(multiple_match(color_vector[i])); //save the value into the vector

        value_vector.push(tolerance_match(color_vector[i+1]));


        println!("\nResistor Value: {}{}{}{} +/- {}%",value_vector[0],value_vector[1],value_vector[2],value_vector[3],value_vector[4]);
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

///The Six band Resistor Function
fn six_band_resistor(mut color_vector: Vec<&str>){

    let mut reversi = String::new();
    let mut active = true;

    while active{

        let mut value_vector : Vec<&str>=Vec::new();
        let mut i = 0;
        reversi.clear();

        while i < 3 {//handles digit 1 and 2

            let result = number_match(color_vector[i]);
            value_vector.push(result);

            i+=1;
        }

        value_vector.push(multiple_match(color_vector[i])); //save the value into the vector

        value_vector.push(tolerance_match(color_vector[i+1]));

        value_vector.push(temp_match(color_vector[i+2]));

        println!("\nResistor Value: {}{}{}{} +/- {}% , {} ppm/K",value_vector[0],value_vector[1],value_vector[2],value_vector[3],value_vector[4],value_vector[5]);
        println!("Reverse colors to fix errors? Y/N?");

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