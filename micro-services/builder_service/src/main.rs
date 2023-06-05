mod controllers;
mod middleware;
mod models;
use actix_files as fs;
use actix_web::{
    web::{scope, JsonConfig, ServiceConfig},
    App, HttpServer,
};
use load_dotenv::load_dotenv;
use middleware::{
    cors_middelware::init_cors,
    logging_middelware::{get_subscriber, init_subscriber},
};
use models::{
    CategoriesCollection, FeaturesCollection, ProjectsCollection, PrototypesCollection,
    TemplatesCollection,
};
use mongodb::{options::ClientOptions, Client};
use std::env;
use tracing_actix_web::TracingLogger;

#[derive(Clone)]
pub struct CollectionsContainer {
    #[allow(dead_code)]
    category: CategoriesCollection,
    feature: FeaturesCollection,
    project: ProjectsCollection,
    prototype: PrototypesCollection,
    template: TemplatesCollection,
}

pub struct AppState {
    #[allow(dead_code)]
    container: CollectionsContainer,
}

async fn establish_connection() -> CollectionsContainer {
    load_dotenv!();
    let client_options = ClientOptions::parse(env!("BUILDER_DATABASE_URL"))
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(env!("BUILDER_DATABASE"));
    CollectionsContainer {
        category: CategoriesCollection::new(db.collection(env!("CATEGORIES_COLLECTION"))),
        feature: FeaturesCollection::new(db.collection(env!("FEATURES_COLLECTION"))),
        project: ProjectsCollection::new(db.collection(env!("PROJECTS_COLLECTION"))),
        prototype: PrototypesCollection::new(db.collection(env!("PROTOTYPES_COLLECTION"))),
        template: TemplatesCollection::new(db.collection(env!("TEMPLATES_COLLECTION"))),
    }
}

pub fn init_services(cfg: &mut ServiceConfig) {
    cfg
        .service(controllers::create_category)
        .service(controllers::update_category)
        .service(controllers::get_all_categories)
        .service(controllers::get_category_by_id)
        .service(controllers::delete_category)
        .service(controllers::create_feature)
        .service(controllers::update_feature)
        .service(controllers::delete_feature)
        .service(controllers::get_all_features)
        .service(controllers::get_feature_by_id)
        .service(controllers::add_feature_wireframe)
        .service(controllers::delete_feature_wireframe)
        .service(controllers::create_template)
        .service(controllers::update_template)
        .service(controllers::get_template_by_id)
        .service(controllers::get_all_templates)
        .service(controllers::delete_template)
        .service(controllers::update_template_feature)
        .service(controllers::get_templates_by_categories_id)
        .service(controllers::add_template_specification)
        .service(controllers::add_prototype)
        .service(controllers::get_prototype_by_template_id)
        .service(controllers::update_prototype)
        .service(controllers::add_project)
        .service(controllers::get_project_by_id)
        .service(controllers::get_all_project_by_client_id)
        .service(controllers::change_project_state)
        .service(controllers::get_all_projects)
        .service(controllers::update_project)
        .service(controllers::generate_project_specification)
        .service(controllers::add_full_build_project)
        .service(controllers::add_proposal_project)
        .service(controllers::add_mvp_project)
        .service(controllers::add_design_project);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "5002".to_string())
        .parse()
        .expect("PORT must be a number");
    let subscriber = get_subscriber("app".into(), "info".into());
    init_subscriber(subscriber);
    let collections = establish_connection().await;
    println!("🚀 Server ready at http://127.0.0.1:5002");
    HttpServer::new(move || {
        let collection_container = collections.clone();
        App::new()
            .wrap(init_cors())
            .wrap(TracingLogger)
            .data(AppState {
                container: collection_container,
            })
            .app_data(JsonConfig::default().limit(4096 * 512))
            //2MO
            // limit(1024 * 1024 * 50))//50MO
            .service(scope("/api/v1/builder/").configure(init_services))
    })
    .bind(("0.0.0.0".to_string(), port))?
    .run()
    .await
}
