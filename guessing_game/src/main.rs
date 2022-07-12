use std::io; //We want to use the input/output library from the STL
use rand::Rng; //Imported the external library "rand" to use RNG
use std::cmp::Ordering; //imported the cmp library and its child library


fn main() {


    let secret_number = rand::thread_rng().gen_range(1,101);
    //?? woah that's a lot of things all at once! Help me out here!
    //Sure thing! we are accessing the rand library and within it the 
    //thread_rng() which is a local number generator, and with that 
    //local number generator we can use the gen_range function to generate
    //a number between 1(inclusive) and 101(exclusive), we want the user to
    //guess a number between 1-100, so that's why we did so!
    println!("Welcome to the Number Guessing Game!✿◠‿◠");
    println!("Now how about you go and give me a number between (1-100)?");

    loop 
    {
    
    let mut guess_number = String::new();
    //mut allows the variable to be reassigned in the future, and the ::new()
    //method allows us to create the object of a "String" from its library.

    println!("Guess the number...Waiting for your input...");

    io::stdin().read_line(&mut guess_number)
    .expect("Something went wrong here.");
    //What this is doing, is it uses the stdin() object of the io library
    //and uses the function read_line() to put that input into our guess_number
    //variable. the read_line function returns a RESULT object, which allows
    //for some kind of error handling message

    let guess_number: u32 = match guess_number.trim().parse() {

        Ok(all_good) => all_good,
        Err(_) => {println!("Give me a valid number!\n");continue;}
        //If it was an OK pass, we simply just return the number
        //if not, we check for Errors "the _ stands for everything, sort of
        //like a catch all"
    };

    //we are comparing guess # and secret # and matching them to the "arms"
    //of the possisble results of this result:which returns us Ordering,an enum
    //and an enum is something that has fixed possible values. For Ordering we have:
    match guess_number.cmp(&secret_number) {

        Ordering::Less => println!("Too Small..."),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {println!("You got it! Hooray!");
                            break;}
    }

    //when we compile here, we are getting a mismatch error for comparison,
    //because we are trying to compare an int(secret_number) to
    //our guess number(technically a string) so we must do some type conversion:

    //trim() removes the whitespace because when we press ENTER on ourkeyboard
    //the input becomes [input value] \n. .parse() allows us to parse it into 
    //our declare type we want, in our case, an unsigned integer 32bit.
    //This is quite nice to work with not so large, positive numbers!!@_@

    //but the .parse() can also fail, which returns a RESULT of sorts.
    //as such we can do an expect...
    //BUT can we do better? If the user inputeted an invalid guess "number",
    //then all we'll get it a message say ing something went wrong. Instead,
    //what if we just made them keep asking for a number until they give us
    //a right one? HERE is where MATCH comes to the rescue. The MATCH can return
    //something that it went OK or that there was an error: So we can match:
    }

}
