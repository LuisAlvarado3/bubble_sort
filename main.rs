/*
git config --global user.email "you@example.com"
git config --global user.name "Your Name"
git config --global user.username "Your Name"

git init
git add .
git commit -m "message"
git remote add origin https://github.com/LuisAlvarado3/bubble_sort.git
git remote -v
git push origin master

Program: bubble_sort
Author : José Luis Márquez Alvarado
Date   : 14/03/2023
version: 1.0
*/

use std::io::{
    stdin ,
    stdout,
    Write
};

fn main(){
    let mut colection1: Vec<f32> = Vec::new();
    //let mut colection2: Vec<f32> = Vec::new();
    let quantity_numbers: u32 = input_u32("Insert your amount numbers: ");

    if quantity_numbers > 0 {
        for item in 1 .. quantity_numbers+1 {
            let comment: String = (item).to_string()+": ";
            colection1.push( input_f32( &comment ) );
        }

        bubble_sort( &mut colection1 );
    }

    println!("Colection: {:?}", colection1);
}

fn bubble_sort( colection: &mut Vec<f32> ) {
    let mut usize1: usize = 0;
    let mut usize2: usize;
    let mut auxiliar: f32;

    while usize1 < colection.len()-1 {
        usize2 = 0;
        while usize2 < colection.len()-1 {
            if colection[ usize2 ] > colection[ usize2+1 ] { 
                auxiliar = colection[ usize2 ];
                colection[ usize2   ] = colection[ usize2+1 ];
                colection[ usize2+1 ] = auxiliar;

            }
            usize2 += 1;
        }
        
        usize1+= 1;
    }

}

fn input_u32( comment: &str ) -> u32 {
    let u32_number: u32 = loop {
        let value: String = input( &comment );
        let value: u32 = match value.parse() {
            Ok( number) => number,
            Err(  _   ) => {
                println!( "Please insert a integer number..." );
                continue;
            }
        };
        break value;
    };

    u32_number
} 

fn input_f32( comment: &str ) -> f32 {
    let f32_number: f32 = loop {
        let value: String = input( &comment );
        let value: f32 = match value.parse() {
            Ok( number ) => number,
            Err(   _   ) => {
                println!("Please insert a floatingg number...");
                continue;
            }
        };
        break value;
    };

    f32_number
}

fn input( comment: &str) -> String {
    print!( "{}", comment );
    stdout().flush().unwrap();
    let mut value: String = String::new();
    stdin()
        .read_line( &mut value )
        .expect("Failed to read line");
    (&value.trim()).to_string()
}