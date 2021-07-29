use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod configuration;
mod api;

mod configuration_tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:18086";
    let app_state = web::Data::new( AppState::new());

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new().service(hi)
        .route("/configuration", web::get().to(api::get_config))
    })
    .bind(url)?
    .run()
    .await
}

pub struct AppState {
    shard_collection: configuration::ShardsCollection,
}

impl AppState {
    fn new() -> AppState {
        AppState {shard_collection: AppState::initialize_configuration()}
     }
    
    /// initialize cluster configuration
    fn initialize_configuration() -> configuration::ShardsCollection {
        configuration::ShardsCollection::new()
    }
}


/// Healthcheck endpoint
#[get("/avtan")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("
                        ~-.
                        ,,,;            ~-.~-.~-
                    (.../           ~-.~-.~-.~-.~-.
                < } O~`, ,        ~-.~-.~-.~-.~-.~-.
                    (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
                        ;    T     ~-.~-.~-.~-.~-.~-.~-.
                      ;   {_.~-.~-.~-.~-.~-.~-.~
                    ;:  .-~`    ~-.~-.~-.~-.~-.
                    ;.: :'    ._   ~-.~-.~-.~-.~-
                    ;::`-.    '-._  ~-.~-.~-.~-
                    ;::. `-.    '-,~-.~-.~-.
                        ';::::.`''-.-'
                        ';::;;:,:'
                            '||T
                            / |
                          __   _
           AVTAN KEEPER IS RUNNING!!! KOKOKOKO!!!!! ;)
    ")
}