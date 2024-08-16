use igniter::api::containers;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[macro_use]
extern crate rocket;

// Default Rocket launch
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/v1",
            openapi_get_routes![containers::run_container, containers::inspect_container],
        )
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
