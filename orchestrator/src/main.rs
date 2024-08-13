use std::{sync::Mutex, thread};

use lazy_static::lazy_static;
use orchestrator::{
    igniter_ctr::rest_ctr::IgniterRestClient, orchestrator_conf::OrchestratorConfig,
};
use rocket::{launch, routes};

lazy_static! {
    static ref ORCHESTRATORCONFIG: Mutex<OrchestratorConfig> =
        Mutex::new(OrchestratorConfig::from_file("config.yaml"));
}

#[launch]
fn rocket() -> _ {
    thread::spawn(|| init());
    rocket::build().mount("/", routes![])
}

/* run containers and executable from config file */
fn init() {
    let orchestrator_config = ORCHESTRATORCONFIG.lock().unwrap();
}
