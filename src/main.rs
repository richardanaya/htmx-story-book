use axum::{
    debug_handler,
    extract::{Form, State},
    http::{header, StatusCode},
    http::header::COOKIE,
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::services::ServeDir;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize}; 
use serde_json::json;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use dotenvy::dotenv;
use std::env;

fn get_jwt_secret() -> Vec<u8> {
    dotenv().ok();
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file")
        .into_bytes()
}

#[derive(Debug, Serialize, Deserialize)] 
struct Claims {
    sub: String,  // username
    exp: usize,   // expiration time
    iat: usize,   // issued at
}

#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    summary: String,
    pages: Vec<Page>,
    starting_page: u32,
}

#[derive(Debug, Clone)]
struct Page {
    id: u32,
    content: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Clone)]
struct Choice {
    text: String,
    target_page_id: u32,
}

struct AppState {
    handlebars: Handlebars<'static>,
    library: Vec<Book>,
}

fn generate_fake_library() -> Vec<Book> {
    vec![
        Book {
            id: 1,
            title: "The Haunted Mansion".to_string(),
            summary: "Explore a spooky mansion full of secrets".to_string(),
            starting_page: 101,
            pages: vec![
                Page {
                    id: 101,
                    content: "You stand before a creaky old mansion. Do you:".to_string(),
                    choices: vec![
                        Choice {
                            text: "Enter through the front door".to_string(),
                            target_page_id: 102,
                        },
                        Choice {
                            text: "Sneak around to the back".to_string(),
                            target_page_id: 103,
                        },
                    ],
                },
                Page {
                    id: 102,
                    content: "The front door creaks open. Inside is a dark hallway. Do you:".to_string(),
                    choices: vec![
                        Choice {
                            text: "Light a match and explore".to_string(),
                            target_page_id: 104,
                        },
                        Choice {
                            text: "Feel your way in the dark".to_string(),
                            target_page_id: 105,
                        },
                    ],
                },
                Page {
                    id: 103,
                    content: "You find a broken window at the back. Do you:".to_string(),
                    choices: vec![
                        Choice {
                            text: "Climb through carefully".to_string(),
                            target_page_id: 106,
                        },
                        Choice {
                            text: "Look for another way in".to_string(),
                            target_page_id: 101,
                        },
                    ],
                },
            ],
        },
        Book {
            id: 2,
            title: "Space Station Omega".to_string(),
            summary: "A sci-fi adventure in deep space".to_string(),
            starting_page: 201,
            pages: vec![
                Page {
                    id: 201,
                    content: "The space station alarms are blaring! Do you:".to_string(),
                    choices: vec![
                        Choice {
                            text: "Head to the control room".to_string(),
                            target_page_id: 202,
                        },
                        Choice {
                            text: "Check the engineering bay".to_string(),
                            target_page_id: 203,
                        },
                    ],
                },
                Page {
                    id: 202,
                    content: "You reach the control room. The main console is sparking! Do you:".to_string(),
                    choices: vec![
                        Choice {
                            text: "Attempt to repair it".to_string(),
                            target_page_id: 204,
                        },
                        Choice {
                            text: "Call for help on the comms".to_string(),
                            target_page_id: 205,
                        },
                    ],
                },
                Page {
                    id: 203,
                    content: "In engineering, you see a coolant leak. Do you:".to_string(),
                    choices: vec![
                        Choice {
                            text: "Try to seal the leak".to_string(),
                            target_page_id: 206,
                        },
                        Choice {
                            text: "Evacuate the area".to_string(),
                            target_page_id: 207,
                        },
                    ],
                },
            ],
        },
    ]
}

#[tokio::main]
async fn main() {
    let mut handlebars = Handlebars::new();

    // Register templates
    handlebars
        .register_template_file("index", "templates/index.hbs")
        .expect("Failed to register index template");
    handlebars
        .register_template_file("signature", "templates/signature.hbs")
        .expect("Failed to register signature partial");
    handlebars
        .register_template_file("login", "templates/login.hbs")
        .expect("Failed to register login partial");
    handlebars
        .register_template_file("logged_in", "templates/logged_in.hbs")
        .expect("Failed to register logged in template");
    handlebars
        .register_template_file("non_logged_in_content", "templates/pages/non_logged_in_content.hbs")
        .expect("Failed to register non logged in content template");
    handlebars
        .register_template_file("logged_in_content", "templates/pages/logged_in_content.hbs")
        .expect("Failed to register logged in content template");

    let state = Arc::new(AppState {
        handlebars,
        library: generate_fake_library(),
    });

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(index_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .with_state(state);

    println!("Server starting on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

fn validate_credentials(username: &str, password: &str) -> bool {
    // For now, we have a single hardcoded user
    // In a real application, this would check against a database
    username == "richard" && password == "secret"
}

#[debug_handler]
async fn login_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    if validate_credentials(&form.username, &form.password) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
            
        let claims = Claims {
            sub: form.username.clone(),
            exp: now + 3600, // Token expires in 1 hour
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&get_jwt_secret())
        ).unwrap();

        let data = json!({
            "username": form.username,
            "error": null
        });
        
        let rendered = state
            .handlebars
            .render("logged_in", &data)
            .expect("Failed to render logged in template");

        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::SET_COOKIE,
                format!("auth={}; Path=/; HttpOnly; SameSite=Strict", token),
            )
            .header(header::CONTENT_TYPE, "text/html")
            .header("HX-Trigger", "login-success")
            .header("HX-Refresh", "true")  // Add this header for full page refresh
            .body(rendered.into())
            .unwrap()
    } else {
        let rendered = state
            .handlebars
            .render(
                "login",
                &json!({
                    "error": "Invalid username or password"
                }),
            )
            .expect("Failed to render login template");
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    }
}

async fn logout_handler(State(state): State<Arc<AppState>>) -> Response {
    let data = json!({
        "title": "Storybuilder",
        "heading": "Storybuilder",
    });
    let rendered = state
        .handlebars
        .render("index", &data)
        .expect("Failed to render template");

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::SET_COOKIE,
            "auth=; Path=/; HttpOnly; Max-Age=0"
        )
        .header(header::CONTENT_TYPE, "text/html")
        .body(rendered.into())
        .unwrap()
}


async fn index_handler(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Html<String> {
    let mut data = json!({
        "title": "Storybuilder",
        "heading": "Storybuilder",
        "state": {
            "library": &state.library
        }
    });

    if let Some(cookie) = headers.get(COOKIE) {
        if let Some(cookie_str) = cookie.to_str().ok() {
            if let Some(token) = cookie_str
                .split(';')
                .find(|s| s.trim().starts_with("auth="))
                .and_then(|s| s.trim().strip_prefix("auth="))
            {
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&get_jwt_secret()),
                    &Validation::default()
                ) {
                    data["username"] = json!(token_data.claims.sub);
                }
            }
        }
    }

    let rendered = state
        .handlebars
        .render("index", &data)
        .expect("Failed to render template");

    Html(rendered)
}
