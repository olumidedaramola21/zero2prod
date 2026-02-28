use actix_web::{App, HttpResponse, HttpServer, Responder, web};
// use zero2prod::health_check;


#[actix_web::test]    // test attribute macro
async fn health_check_works() {
  // Arrange
  spawn_app().await.expect("Failed to spawn our app!");

  let client = reqwest::Client::new();

  // Act phase
  let response = client
      .get("http://127.0.0.1:8000/health_check")
      .send()
      .await
      .expect("Failed to execute request.");
      
  // Assert phase
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());

}


async fn spawn_app() -> std::io::Result<()> {
  // let server = HttpServer::new(|| {
  //   App::new()
  //       .route("/health_check", web::get().to(health_check))
  // })
  // .bind("127.0.0.1:8000") ?
  // .run
  todo!()
}
