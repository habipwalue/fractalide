// Simple main.rs to run the web server example
use std::io;
use std::collections::HashMap;

fn main() {
    println!("Fractalide Web Server Example");
    println!("============================");
    println!("This is a simplified example without the full Fractalide runtime");
    println!("In a real implementation, this would use the Fractalide dataflow architecture");
    
    // Simulate the web server
    simulate_web_server();
}

// Simplified HTTP structures for the simulation
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

fn simulate_web_server() {
    println!("\nSimulating a web server running on http://localhost:8080");
    println!("Enter a path to request (or 'q' to quit):");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let path = input.trim();
        if path == "q" {
            println!("Web server stopped.");
            break;
        }
        
        // Create a simulated HTTP request
        let request = HttpRequest {
            method: "GET".to_string(),
            path: path.to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("User-Agent".to_string(), "Fractalide-Example/1.0".to_string());
                headers.insert("Accept".to_string(), "text/html,application/json".to_string());
                headers
            },
            body: "".to_string(),
        };
        
        // Simulate node processing
        println!("\n[Node: HttpServer] Received request: {} {}", request.method, request.path);
        
        // Simulate router
        println!("[Node: Router] Routing request for path: {}", request.path);
        let route_result = match request.path.as_str() {
            "/" => {
                println!("[Node: Router] Route matched: / -> StaticFileHandler (index.html)");
                ("StaticFileHandler", "index.html")
            },
            "/api/users" => {
                println!("[Node: Router] Route matched: /api/users -> ApiHandler (users)");
                ("ApiHandler", "users")
            },
            "/api/products" => {
                println!("[Node: Router] Route matched: /api/products -> ApiHandler (products)");
                ("ApiHandler", "products")
            },
            _ => {
                if request.path.starts_with("/static/") {
                    println!("[Node: Router] Route matched: /static/* -> StaticFileHandler");
                    ("StaticFileHandler", &request.path[8..])
                } else {
                    println!("[Node: Router] No route matched -> NotFoundHandler");
                    ("NotFoundHandler", "")
                }
            }
        };
        
        // Simulate handler
        let response = match route_result.0 {
            "StaticFileHandler" => {
                println!("[Node: StaticFileHandler] Serving file: {}", route_result.1);
                HttpResponse {
                    status_code: 200,
                    status_text: "OK".to_string(),
                    headers: {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "text/html".to_string());
                        headers
                    },
                    body: format!("<html><body><h1>Static File: {}</h1></body></html>", route_result.1),
                }
            },
            "ApiHandler" => {
                println!("[Node: ApiHandler] Handling API request for: {}", route_result.1);
                HttpResponse {
                    status_code: 200,
                    status_text: "OK".to_string(),
                    headers: {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "application/json".to_string());
                        headers
                    },
                    body: format!("{{\"resource\": \"{}\", \"count\": 42}}", route_result.1),
                }
            },
            "NotFoundHandler" => {
                println!("[Node: NotFoundHandler] Generating 404 response");
                HttpResponse {
                    status_code: 404,
                    status_text: "Not Found".to_string(),
                    headers: {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "text/html".to_string());
                        headers
                    },
                    body: "<html><body><h1>404 - Page Not Found</h1></body></html>".to_string(),
                }
            },
            _ => unreachable!(),
        };
        
        // Simulate response
        println!("[Node: HttpServer] Sending response: {} {}", response.status_code, response.status_text);
        println!("[Node: Logger] {} {} {} {}", 
            request.method, 
            request.path, 
            response.status_code, 
            response.body.len());
        
        // Display response to user
        println!("\nResponse:");
        println!("HTTP/1.1 {} {}", response.status_code, response.status_text);
        for (name, value) in &response.headers {
            println!("{}: {}", name, value);
        }
        println!("");
        println!("{}", response.body);
    }
}