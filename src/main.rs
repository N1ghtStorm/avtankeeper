use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod configuration;
mod api;
mod relations;

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
    distributed_edges_map: relations::EdgeMap,
    distributed_vertex_map: relations::VertexMap,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            shard_collection: Self::initialize_configuration(), 
            distributed_edges_map: Self::initialize_edges(), 
            distributed_vertex_map: Self::initialize_vertexes()
        }
     }
    
    /// initialize cluster configuration
    fn initialize_configuration() -> configuration::ShardsCollection {
        configuration::ShardsCollection::new()
    }

    fn initialize_edges() -> relations::EdgeMap {
        relations::EdgeMap::new()
    }

    fn initialize_vertexes() -> relations::VertexMap {
        relations::VertexMap::new()
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