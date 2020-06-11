mod domain;

use actix_web::{web, App, HttpResponse, HttpServer};
use listenfd::ListenFd;
use domain::{UserCreation, AppState};

fn create_user(creation: web::Json<UserCreation>, state: web::Data<AppState>) -> HttpResponse {
    let new_user = state.into_inner().create_user(creation.0);
    HttpResponse::Created().json(new_user)
}

fn get_user(path: web::Path<(u32,)>, state: web::Data<AppState>) -> HttpResponse {
    let user_id = path.0;

    match state.into_inner().get_user_by_id(user_id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::default());

    let mut server = HttpServer::new(move || {
        App::new().app_data(state.clone()).service(
            web::scope("/users")
                .route("", web::post().to(create_user))
                .route("/{id}", web::get().to(get_user))
        )
    });

    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
