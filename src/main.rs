use std::io;

fn main() {
    println!("Lirsp v0.0.1"); // Should be a constant somewhere or something
    println!("Ctrl + C to Exit.");

    let prompt = "lirsp";

    let mut buf = vec![];

    loop {
        buf.clear();
        print!("{}> ", prompt);

        let input = io::stdin().read_line().ok().expect("No input");

        for c in input.chars() {
            match c {
                '(' | ')' => buf.push(c),
                '\n' => {},
                _ => println!("character `{}` not valid", c),
            }
        }

        for c in buf.iter() {
            print!("{}", c);
        }
        println!("");
        
        //print!("\x1b[0G\x1b[10C"); // set to 0 then to right by 10
        //print!("\x1b[10C"); // move to right by 10

        //let input = io::stdin().read_line().ok().expect("Failed to readline");

        // Loop over `input` and search for certain characters?
        // not sure what the best way to do this is.
        // Should something trigger every keydown? can I do that?

        // print!("\x1b[0G");
    }
}
