# Improper Input Validation in Rust (actix-web) 🦀

This project is a hands-on demonstration of an improper input validation vulnerability and its mitigation in a Rust web application built with the actix-web framework.

The application exposes two endpoints to show the direct impact of server-side validation: one that is vulnerable to improper input and another that is secured against it.

Understanding the Vulnerability
Improper input validation occurs when an application fails to correctly validate untrusted input.  Untrusted input is any data that comes from an external source, such as user form submissions, query strings, or API requests. 


This vulnerability arises when untrusted input is not checked for:

Syntactical Correctness: Ensuring the data is in the correct format the application expects (e.g., a number is a number, not text). 
Semantic Correctness: Ensuring the data makes sense within the business context of the application (e.g., a shopping basket quantity is a positive number). 
Failing to validate input properly can allow an attacker to affect the application's behavior, leading to data manipulation, unintended execution flow, or more severe exploits.  In this demo's scenario, a user can order a negative quantity of an item to receive store credit. 


Project Demo
This application runs a single server with two distinct endpoints to demonstrate the vulnerability and the fix.

Prerequisites
The Rust toolchain (install via rustup.rs)
How to Run
Clone the repository and navigate into the project directory.
Build the project:
Bash

cargo build
Run the server:


```bash
cargo run
```

The server will start and listen on http://127.0.0.1:8080. You will see the following output confirming the available endpoints:
Starting server...
Vulnerable endpoint: POST http://127.0.0.1:8080/vulnerable/basket
Fixed endpoint:    POST http://127.0.0.1:8080/fixed/basket
Demonstration Steps
You can use a tool like curl to send requests to the server.

1. Test the VULNERABLE Endpoint
Send a POST request with a negative quantity to the /vulnerable/basket endpoint.

Bash

curl -X POST http://localhost:8080/vulnerable/basket \
     -H "Content-Type: application/json" \
     -d '{"item_id": 101, "quantity": -99}'
Expected Result:
You will receive a 200 OK response, and the server log will show that the invalid data was processed.

[VULNERABLE] Adding item 101 with quantity -99. This should not happen with negative quantities!
This shows the vulnerability: the application processed nonsensical data because it did not perform semantic validation.

2. Test the FIXED Endpoint
Send the exact same malicious payload to the /fixed/basket endpoint.

Bash

curl -X POST http://localhost:8080/fixed/basket \
     -H "Content-Type: application/json" \
     -d '{"item_id": 101, "quantity": -99}'
Expected Result:
You will receive a 400 Bad Request response with an error message. The server log will show that the request was rejected.

[FIXED] Rejected request with invalid quantity: -99
This demonstrates the fix: the application's business logic is protected because the input was validated before being processed.

Mitigation and Key Takeaways
The vulnerability is mitigated by performing validation on the server before acting on the data. 

The Fix: The fix was implemented by using the validator crate. By adding a validation rule (#[validate(range(min = 1))]) to our BasketRequest data structure and calling the .validate() method in our handler, we enforce the business rule that the quantity must be a positive number.
Use Libraries: Implementing your own validation logic can be a tedious and error-prone process.  It is highly recommended to use well-vetted libraries for this task. 

Server-Side is Key: Input validation must always be enforced on the server.  Client-side validation is for user experience only and is trivially bypassable. 