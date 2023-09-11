fn main(){
    let string1 = String::from("hello");
    let string2 = String::from(" world");
    
    let concatenated_strings= concatenate_strings(&string1, &string2);
    println!("{}", concatenated_strings);
}
fn concatenate_strings(x: &str, y: &str)->String{
    let mut result = String::new();
    result.push_str(x);
    result.push_str(y);
    return result;
}