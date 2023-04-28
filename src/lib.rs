use db::{
    app::{account::D1AccountDatabase, task::D1TaskDatabase},
    service::Service,
};
use router::{delete_task, get_task, patch_task, post_task, create_account, login};
use worker::*;

mod db;
pub mod router;
pub mod util;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

fn get_service(env: &Env) -> Result<Service<D1TaskDatabase, D1AccountDatabase>> {
    env.d1("DB")
        .and_then(|db| env.d1("DB").map(|db2| (db, db2)))
        .map(|(db, db2)| Service::new(D1TaskDatabase::new(db), D1AccountDatabase::new(db2)))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    util::hooks::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/task", |req, ctx| async move {
            post_task(req, &get_service(&ctx.env)?).await
        })
        .get_async("/task", |_, ctx| async move {
            get_task(&get_service(&ctx.env)?).await
        })
        .patch_async("/task", |req, ctx| async move {
            patch_task(req, &get_service(&ctx.env)?).await
        })
        .delete_async("/task/:id", |_, ctx| async move {
            delete_task(&ctx, &get_service(&ctx.env)?).await
        })
        .post_async("/account/signup", |req, ctx| async move {
            create_account(req, &get_service(&ctx.env)?).await
        })
        .post_async("/account/login", |req, ctx| async move {
            login(req, &get_service(&ctx.env)?).await
        })
        .run(req, env)
        .await
}
