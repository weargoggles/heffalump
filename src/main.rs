extern crate actix_web;
extern crate activitypub;
extern crate tokio_postgres;
extern crate actix;
extern crate tokio;
extern crate postgres_native_tls;
extern crate native_tls;

use actix::{System};
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use activitypub::actor::{Person};
use activitypub::collection::{OrderedCollection};
use std::env;

mod db_pg;
use crate::db_pg::{Database};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello nicely nicely world!\n")
}

fn actor(req: HttpRequest, info: web::Path<(String,)>) -> impl Responder {
    let mut person = Person::default();

    let inbox_url_string = match req.url_for("inbox", &[&info.0]) {
        Ok(url) => url.into_string(),
        Err(e) => return Err(e),
    };

    let outbox_url_string = match req.url_for("outbox", &[&info.0]) {
        Ok(url) => url.into_string(),
        Err(e) => return Err(e),
    };

    person.ap_actor_props.inbox = inbox_url_string.into();
    person.ap_actor_props.outbox = outbox_url_string.into();
    Ok(web::Json(person))
}

fn inbox() -> impl Responder {
    web::Json(OrderedCollection::default())
}

fn inbox_submit() -> impl Responder {
    "Thanks!"
}

fn outbox() -> impl Responder {
    web::Json(OrderedCollection::default())
}

fn outbox_submit() -> impl Responder {
    "Thanks!"
}

fn main() -> std::io::Result<()> {
    let sys = System::builder().stop_on_panic(true).build();

    let server = HttpServer::new(|| {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "couldn't read env var DATABASE_URL").unwrap();
        // let addr = Database::connect(&database_url).unwrap();
        App::new()
            // .data(addr)
            .route("/", web::get().to(index))
            .service(
                web::resource("/inbox/{name}/")
                    .name("inbox")
                    .guard(guard::Get())
                    .to(inbox)
            )
            .service(
                web::resource("/inbox/{name}/")
                    .name("inbox")
                    .guard(guard::Post())
                    .to(inbox_submit)   
            )
            .service(
                web::resource("/outbox/{name}/")
                    .name("outbox")
                    .guard(guard::Get())
                    .to(outbox)
            )
            .service(
                web::resource("/outbox/{name}/")
                    .name("outbox")
                    .guard(guard::Post())
                    .to(outbox_submit)   
            )
            .route("/{name}", web::get().to(actor))
    }).bind("127.0.0.1:8088")?
    .start();
    
    sys.run()
}
