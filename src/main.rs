use std::{
    env,
    io::{Write, stdin, stdout},
    path::Path,
    process::{Child, Command, Stdio},
};

fn main() {
    loop {
        println!(">");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline, which trim removes
        // let mut parts = input.trim().split_whitespace();
        // println!("Parts: {:?}", parts);

        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;
        // println!("Command: {}", command);

        while let Some(command) = commands.next() {
            // println!("Args: {:?}", args);

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                "exit" => {
                    return;
                }
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }

                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait();
        }
    }
}
