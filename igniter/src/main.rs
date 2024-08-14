use igniter::api::containers;
use igniter::config::Config;
use lazy_static::lazy_static;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use std::io::{Error as IoError, ErrorKind};
use std::{
    error::Error,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

#[macro_use]
extern crate rocket;

lazy_static! {
    static ref CONFIG: Config = Config::from_file("config.yaml");
}

// Default Rocket launch
#[launch]
async fn rocket() -> _ {
    let settings = rocket_okapi::settings::OpenApiSettings::new();

    rocket::build()
        .mount("/v1", openapi_get_routes![index, containers::run_container])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                urls: vec![UrlObject::new("v1", "../v1/openapi.json")],
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("v1", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}

// This endpoint turns the application on in the port passed in the request
#[openapi(tag = "port")]
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
