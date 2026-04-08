fn main() {
    use std::io;
    //lesson 3 start topic: strings
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut surname = String::new();
    io::stdin().read_line(&mut surname).expect("Failed to read line");
    
    let name = name.trim();
    let surname = surname.trim();
    let firstpart;
    let secondpart;

    if name.len() < 3{
        firstpart = name.get(0..name.len()).unwrap().to_string();
    }
    else{
        firstpart = name.get(0..3).unwrap().to_string();
    }

    if surname.len() < 5{
        secondpart = surname.get(0..surname.len()).unwrap().to_string();
    }
    else{
        secondpart = surname.get(0..5).unwrap().to_string();
    }

    let username = secondpart.to_lowercase() + &firstpart.to_lowercase();

    println!("{}", username);

    let faculty_email = username.clone() + "@fit.cvut.cz";
    let general_email = username.clone() + "@cvut.cz";

    println!("{}", faculty_email);
    println!("{}", general_email);
    //lesson 3 end 
}

