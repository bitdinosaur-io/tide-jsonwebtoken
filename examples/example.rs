use tide::Request;
use tide_jsonwebtoken::{ApiKeyMiddleware, Claims};

#[derive(Clone)]
struct State {
    jwt: ApiKeyMiddleware,
}
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let jwt = ApiKeyMiddleware::new("T}HT!x^YwdtuQ}u7984P=)_B--iV^UK6");
    let state = State { jwt: jwt.clone() };
    let mut app = tide::with_state(state);
    app.at("/login").get(|req: Request<State>| async move {
        let token = req
            .state()
            .jwt
            .clone()
            .gen_token("free", "test1", "010101", 1798111872, 1798110872)
            .unwrap();
        Ok(format!("Your token: {:?}", token))
    });
    app.at("/user/:id")
        .get(|_| async { Ok("No User JWT Check Login") });

    app.at("/name/")
        .with(jwt.clone())
        .get(|req: Request<State>| async move {
            Ok(format!("Hello, {}!", req.ext::<Claims>().unwrap().uid))
        });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
