// src/main.rs
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::Deserialize;
use validator::Validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    println!("Vulnerable endpoint: POST http://127.0.0.1:8080/vulnerable/basket");
    println!("Fixed endpoint:    POST http://127.0.0.1:8080/fixed/basket");

    HttpServer::new(|| {
        App::new()
            .service(vulnerable_add_to_basket) // Register the vulnerable route
            .service(fixed_add_to_basket) // Register the fixed route
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// We define a single data structure with the validation rules.
// Both endpoints will use this struct, but only the fixed one will
// enforce the rules.
#[derive(Deserialize, Validate)]
struct BasketRequest {
    item_id: u32,
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    quantity: i32,
}

// --- VULNERABLE ENDPOINT ---
// This handler exposes the security flaw.
#[post("/vulnerable/basket")]
async fn vulnerable_add_to_basket(payload: web::Json<BasketRequest>) -> impl Responder {
    // VULNERABILITY: No call to `payload.validate()` is made.
    // The application blindly trusts the incoming data, even though validation
    // rules are defined on the struct. It simply ignores them.
    println!(
        "[VULNERABLE] Adding item {} with quantity {}. This should not happen with negative quantities!",
        payload.item_id, payload.quantity
    );
    HttpResponse::Ok().body("VULNERABLE: Item was added to basket.")
}

// --- FIXED ENDPOINT ---
// This handler correctly implements input validation.
#[post("/fixed/basket")]
async fn fixed_add_to_basket(payload: web::Json<BasketRequest>) -> impl Responder {
    // FIX: The payload is validated before any processing occurs.
    if let Err(e) = payload.validate() {
        // If validation fails, reject the request with a clear error.
        println!(
            "[FIXED] Rejected request with invalid quantity: {}",
            payload.quantity
        );
        return HttpResponse::BadRequest().body(e.to_string());
    }

    // This code is only reachable if the data is valid.
    println!(
        "[FIXED] Successfully added item {} with quantity {}",
        payload.item_id, payload.quantity
    );
    HttpResponse::Ok().body("FIXED: Item added to basket.")
}
