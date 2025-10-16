mod includes;
use includes::*;

#[derive(Template)]
#[template(path="home.html")]
struct HomePage;

#[derive(Template)]
#[template(path="login.html")]
struct LoginPage;

async fn home() -> Html<String> {
	Html(HomePage.render().unwrap())
}

async fn login_get() -> Html<String> {
	Html(LoginPage.render().unwrap())
}

async fn _login_post() {
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    	.route("/", get(home))
    	.route("/login", get(login_get).post(_login_post));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
	println!("Listening on http://{}", addr);
	    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
