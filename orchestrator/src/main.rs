use lazy_static::lazy_static;
use orchestrator::orchestrator_conf::OrchestratorConfig;
use rocket::{launch, routes};

lazy_static! {
    static ref ORCHESTRATORCONFIG: OrchestratorConfig =
        OrchestratorConfig::from_file("config.yaml");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}
