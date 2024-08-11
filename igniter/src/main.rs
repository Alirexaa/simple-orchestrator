use std::io::{Error as IoError, ErrorKind};
use std::{
    error::Error,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use igniter::config::Config;
use lazy_static::lazy_static;
use rocket::{get, launch, routes};

lazy_static! {
    static ref CONFIG: Config = Config::from_file("config.yaml");
}

// Default Rocket launch
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

// This endpoint turns the application on in the port passed in the request
#[get("/<port>")]
fn index(port: i32) -> String {
    match start_server(port) {
        Ok(()) => {
            println!("STARTED");
            "Server started successfully".to_owned()
        }
        Err(e) => format!("Failed to start server: {}", e),
    }
}

/** Starts the server given a port, the command and path of a given application */
fn start_server(port: i32) -> Result<(), Box<dyn Error>> {
    //create a new cmd instance
    let mut cmd = Command::new("cmd");
    // tells the cmd to run the command in a shell and close the shell when done
    cmd.arg("/C")
        //
        // change directory to the specified path
        .arg(format!("cd {}", CONFIG.path))
        // run start command with port
        .arg(format!("& {} {}", CONFIG.start_command, port))
        // capture the stdout of the program
        .stdout(Stdio::piped())
        // capture the stderr of the program
        .stderr(Stdio::piped());

    // Start the command and get a handle to its standard output and error streams
    let mut child_proc = cmd.spawn().expect("faild to start");

    // Wait for 1 sec
    thread::sleep(Duration::from_millis(1000));

    // Try wait for child proccess to end

    match child_proc.try_wait() {
        //If program exited means it probably failed.
        Ok(Some(status)) => {
            return Err(Box::new(IoError::new(
                ErrorKind::Other,
                format!("proccess exit with status code {}", status),
            )))
        }
        // If it has not ended then asume it starts
        Ok(_) => {
            println!("SERVER SUCCESSFULLY STARTED AT PORT {}", port);
            return Ok(());
        }
        // If any other error happens return it
        Err(err) => {
            return Err(Box::new(IoError::new(ErrorKind::Other, format!("{}", err))));
        }
    }
}
