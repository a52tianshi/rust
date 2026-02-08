use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    numbers: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://localhost:3000");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn get_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <style>
                body { font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px; }
                form { background: #f5f5f5; padding: 20px; border-radius: 8px; }
                input { width: 100%; padding: 10px; margin: 10px 0; font-size: 16px; }
                button { background: #4CAF50; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; }
                button:hover { background: #45a049; }
                .result { background: #e8f5e9; padding: 15px; margin-top: 20px; border-radius: 8px; }
            </style>
            <h1>GCD Calculator</h1>
            <form action="/gcd" method="post">
                <label for="numbers">Enter numbers (separated by spaces or commas):</label>
                <input type="text" name="numbers" id="numbers" placeholder="e.g., 42 56 98 or 42,56,98" required/>
                <button type="submit">Compute GCD</button>
            </form>
        "#,
    ))
}

async fn post_gcd(form: web::Form<GcdParameters>) -> Result<HttpResponse> {
    // Parse the input string into numbers
    let numbers_str = form.numbers.trim();
    if numbers_str.is_empty() {
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body("<h1>Error: No numbers provided</h1><p><a href=\"/\">Go back</a></p>"));
    }

    // Split by comma or space and parse numbers
    let numbers: Vec<u64> = numbers_str
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<u64>().ok()
            }
        })
        .collect();

    if numbers.is_empty() {
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body("<h1>Error: No valid numbers found</h1><p><a href=\"/\">Go back</a></p>"));
    }

    // Calculate GCD
    let mut d = numbers[0];
    for &m in &numbers[1..] {
        d = gcd(d, m);
    }

    let response = format!(
        r#"
        <title>GCD Result</title>
        <style>
            body {{ font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px; }}
            .result {{ background: #e8f5e9; padding: 20px; border-radius: 8px; margin: 20px 0; }}
            .numbers {{ font-size: 18px; color: #666; }}
            .gcd {{ font-size: 24px; color: #2e7d32; font-weight: bold; }}
            a {{ color: #4CAF50; text-decoration: none; }}
            a:hover {{ text-decoration: underline; }}
        </style>
        <h1>GCD Result</h1>
        <div class="result">
            <p class="numbers">Numbers: {:?}</p>
            <p class="gcd">Greatest Common Divisor: {}</p>
        </div>
        <p><a href="/">Calculate another GCD</a></p>
        "#,
        numbers, d
    );

    Ok(HttpResponse::Ok().content_type("text/html").body(response))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
