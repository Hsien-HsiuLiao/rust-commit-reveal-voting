use std::io;

fn main() {
    println!("Voting has started. Please vote choice1 or choice2");

    println!("Submit vote in the format <your vote> ~ <yoursecret> (example: YES~secret). 
                You can vote multiple times with different secrets");

    let mut vote = String::new();
    
    io::stdin()
        .read_line(&mut vote)
        .expect("Failed to read line");

    
}
