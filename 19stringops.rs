fn main(){
    let sentence = "a quick brown fox jumps over the lazy dog".to_string();
    
    //slicing to get the first 3 characters
    println!("{}",&sentence[0..=7]);

    //concatenate using format
    let description = format!("The title: \n{}",&sentence);
    println!("{}",description);

    //iterate over characters in the same sentence
    for c in sentence.chars(){
        match c{
            'a'| 'e' | 'i' | 'o' | 'u' => println!("Got a vowel: {}",c),
            _=> continue,
        }
    }

    //split and collect into a vector
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    println!("{:?}",words);


    //reverse string
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}",reversed);
}
