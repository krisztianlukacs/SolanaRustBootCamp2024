// Specify the data type "char"
const character_1: char = 'S';
const character_2: char = 'f';
   
// Compiler interprets a single item in quotations as the "char" data type
const smiley_face: char = '😃';

// Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
const string_1: &str = "miley ";

// Specify the data type "str" with the reference syntax "&str"
const string_2: &str = "ace";

fn main() {
    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
}   
