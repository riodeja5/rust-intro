use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";

    // HttpResponse::Ok() はステータスコード200を持つHttpResponseBUilderという構造体を返します。
    // HttpResponseBuilderのbody()という関数にレスポンスのボディを渡すとHttpResponseが返ってきます。
    // 戻り値の型がResultなのでOkで包みます。
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
