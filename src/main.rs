use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::{Connection, PgConnection};
use diesel::migration::MigrationSource;
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use diesel_migrations::FileBasedMigrations;
use todo::{InternalError, Mutation, Query};

async fn index(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    log::debug!("POST request");
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    log::debug!("GET request");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

async fn build_schema(database_url: String) -> Result<Schema<Query, Mutation, EmptySubscription>, InternalError> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build()?;

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool)
        .finish();
    log::debug!("SCHEMA: {:#?}", schema.names());

    Ok(schema)
}

async fn run_migrations(database_url: String) {
    let mut db_connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    let mut migrations = FileBasedMigrations::from_path("./migrations")
        .unwrap_or_else(|_| panic!("Error read migration files"))
        .migrations()
        .unwrap();
    migrations.sort_by_key(|m| m.name().to_string());
    for migration in migrations {
        migration.run(&mut db_connection).unwrap();
    }
}

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));

    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::debug!("DB URL: {:#?}", database_url);

    // run_migrations(database_url.clone()).await;

    let schema = build_schema(database_url.clone()).await?;

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
