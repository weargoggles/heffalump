extern crate actix_web;
extern crate activitypub;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use activitypub::actor::{Person};
use activitypub::collection::{OrderedCollection};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello nicely nicely world!\n")
}

fn index2(req: HttpRequest, info: web::Path<(String,)>) -> impl Responder {
    let mut person = Person::default();

    let inbox_url_string = match req.url_for("inbox", &[&info.0]) {
        Ok(url) => url.into_string(),
        Err(e) => return Err(e)
    };

    person.ap_actor_props.inbox = inbox_url_string.into();
    Ok(web::Json(person))
}

fn inbox() -> impl Responder {
    web::Json(OrderedCollection::default())
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::resource("/inbox/{name}")
                    .name("inbox")
                    .guard(guard::Get())
                    .to(inbox)
            )
            .route("/{name}", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
