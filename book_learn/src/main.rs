mod mandelbrot;
mod parse;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    println!("Hello, world!");
    let a = 35;
    let b = 49;
    println!("gcd of {} and {} is {}", a, b, gcd(a, b));

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async {
                HttpResponse::Ok().body("Welcome to the GCD Calculator!")
            }))
            .route("/gcd", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Started http server: 127.0.0.1:8080");
    server.bind("127.0.0.1:8080").expect("Failed to bind http server")
        .run().await.expect("Failed to run http server");
}

async fn get_index() -> HttpResponse {
    return HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[derive(Deserialize)]
struct GcdForm {
    n: u64,
    m: u64,
}
async fn post_gcd(form: web::Form<GcdForm>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body(
            "Computing GCD With zero"
        );
    }
    let resp = format!("Got of Numbers {} and {} is <b>{}</b>",
                       form.n, form.m, gcd(form.n, form.m));
    return HttpResponse::Ok().content_type("text/html").body(resp);
}

fn gcd (mut m : u64, mut n : u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 
        3 * 7 * 11 * 13 * 19), 
        3 * 11);
}