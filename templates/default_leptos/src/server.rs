use actix_files::Files;
use actix_web::{middleware, HttpServer};
use leptos::{get_configuration, view};
use leptos_actix::{generate_route_list, LeptosRoutes};
use log::{Level, Metadata, Record};

use crate::app::App;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub async fn run() -> std::io::Result<()> {
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace));
    _ = dotenvy::dotenv();
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr.clone();

    log::info!("Starting server at {}", addr);

    let routes = generate_route_list(|| view! { <App/>});
    println!("{:?}", routes);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;

        let site_root = leptos_options.site_root.clone();

        actix_web::App::new()
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/>},
            )
            .service(Files::new("/", site_root.to_owned()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
