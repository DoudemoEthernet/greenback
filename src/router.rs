use jsonwebtoken::errors::ErrorKind;
use serde::Serialize;
use time::{Duration, OffsetDateTime};
use worker::{console_error, Headers, Request, Response, RouteContext};

use crate::{
    db::{
        entity::{
            account::{Credential, Username},
            task::TaskId,
        },
        error::DatabaseError,
        repository::{account::CredentialRepository, task::TaskRepository},
        service::Service,
    },
    util::{
        jwt::{create_token, validate_token},
        task::{PatchTask, PostTask, ResponseTask},
    },
};

#[derive(Debug, Serialize)]
struct TokenProvider {
    token: String,
    expr: i64,
}

trait HeaderTokenGetter {
    fn get_token(&self) -> worker::Result<Option<String>>;
}

impl HeaderTokenGetter for Headers {
    fn get_token(&self) -> worker::Result<Option<String>> {
        self.get("Authorization")
    }
}

pub async fn post_task<TRepository: TaskRepository, CRepository: CredentialRepository>(
    mut request: Request,
    service: &Service<TRepository, CRepository>,
    token_sugar: &str,
) -> worker::Result<Response> {
    let data: PostTask = request.json().await?;
    let token = request
        .headers()
        .get_token()?
        .map(|token| validate_token(token_sugar, token));
    match token {
        Some(token) => match token {
            Ok(username) => service.create_task(data, username).await.map_or_else(
                |e| match e {
                    DatabaseError::TransactionError(e) => {
                        let message = "Failed to post task";
                        console_error!("{message}: {e}");
                        Response::error(message, 500)
                    }
                    _ => Response::error("unknown error", 500),
                },
                |_| Response::ok(""),
            ),
            Err(e) => match e.kind() {
                ErrorKind::InvalidToken => Response::error("Invalid token", 404),
                _ => {
                    console_error!("failed to validate token {e}");
                    Response::error("internal error", 500)
                }
            },
        },
        None => Response::error("token not found", 404),
    }
}

pub async fn get_task<TRepository: TaskRepository, CRepository: CredentialRepository>(
    request: Request,
    service: &Service<TRepository, CRepository>,
    token_sugar: &str,
) -> worker::Result<Response> {
    let token = request
        .headers()
        .get_token()?
        .map(|token| validate_token(token_sugar, token));
    match token {
        Some(token) => match token {
            Ok(_) => service
                .get_all_tasks(Username::new("dummy-user".to_string()))
                .await
                .map_err(|e| match e {
                    DatabaseError::NotFound(target) => {
                        Response::error(format!("Target not found: {target}"), 404)
                    }
                    DatabaseError::TransactionError(e) => {
                        console_error!("database error: {e}");
                        Response::error("internal error", 500)
                    }
                })
                .map(|tasks| {
                    tasks
                        .iter()
                        .map(|task| ResponseTask::from(task.to_owned()))
                        .collect::<Vec<ResponseTask>>()
                })
                .map(|task| {
                    serde_json::to_string(&task).map_or_else(
                        |e| {
                            console_error!("faield to parse ResponseTask to Json: {e}");
                            Response::error("unknown error", 500)
                        },
                        |json| Response::from_json(&json),
                    )
                })
                .map_or_else(|e| e, |r| r),
            Err(e) => match e.kind() {
                ErrorKind::InvalidToken => Response::error("Invalid token", 404),
                _ => {
                    console_error!("failed to validate token {e}");
                    Response::error("internal error", 500)
                }
            },
        },
        None => Response::error("token not found", 404),
    }
}

pub async fn patch_task<TRepository: TaskRepository, CRepository: CredentialRepository>(
    mut request: Request,
    service: &Service<TRepository, CRepository>,
    token_sugar: &str,
) -> worker::Result<Response> {
    let token = request
        .headers()
        .get_token()?
        .map(|token| validate_token(token_sugar, token));
    let data: PatchTask = request.json().await?;
    match token {
        Some(token) => match token {
            Ok(username) => service.update_task(data, &username).await.map_or_else(
                |e| match e {
                    DatabaseError::TransactionError(e) => {
                        console_error!("faled to update task: {e}");
                        Response::error("Internal error", 500)
                    }
                    _ => Response::error("Unknown error", 500),
                },
                |_| Response::ok(""),
            ),
            Err(e) => match e.kind() {
                ErrorKind::InvalidToken => Response::error("Invalid token", 404),
                _ => {
                    console_error!("failed to validate token {e}");
                    Response::error("internal error", 500)
                }
            },
        },
        None => Response::error("token not found", 404),
    }
}

pub async fn delete_task<TRepository: TaskRepository, CRepository: CredentialRepository>(
    request: Request,
    context: &RouteContext<()>,
    service: &Service<TRepository, CRepository>,
    token_sugar: &str,
) -> worker::Result<Response> {
    let token = request
        .headers()
        .get_token()?
        .map(|token| validate_token(token_sugar, token));
    match token {
        Some(token) => match token {
            Ok(_) => {
                if let Some(id) = context.param("id") {
                    return match TaskId::try_from(id.as_str()) {
                        Err(e) => {
                            console_error!("faield to convert uuid: {e}");
                            Response::error("Invalid id", 400)
                        }
                        Ok(id) => {
                            return service
                                .delete_task(id)
                                .await
                                .map_err(|e| match e {
                                    DatabaseError::NotFound(_) => {
                                        Response::error("target not found", 404)
                                    }
                                    DatabaseError::TransactionError(e) => {
                                        console_error!("failed to delete task: {e}");
                                        Response::error("Internal error", 500)
                                    }
                                })
                                .map_or_else(|e| e, |_| Response::ok("success"))
                        }
                    };
                };

                Response::error("bad request", 400)
            }
            Err(e) => match e.kind() {
                ErrorKind::InvalidToken => Response::error("Invalid token", 404),
                _ => {
                    console_error!("failed to validate token {e}");
                    Response::error("internal error", 500)
                }
            },
        },
        None => Response::error("token not found", 404),
    }
}

pub async fn create_account<TRepository: TaskRepository, CRepository: CredentialRepository>(
    mut request: Request,
    service: &Service<TRepository, CRepository>,
    token_sugar: &str,
) -> worker::Result<Response> {
    let data: Credential = request.json().await?;
    service
        .create_credential(&data)
        .await
        .map(|_| {
            let expr_hour = 6;
            let expr = (OffsetDateTime::now_utc() + Duration::hours(expr_hour)).unix_timestamp();
            create_token(token_sugar, &data.username(), expr_hour)
                .map_err(|e| {
                    console_error!("failed to create token: {e}");
                    Response::error("Internal error", 500)
                })
                .map(|token| TokenProvider { token, expr })
                .map(|provider| {
                    serde_json::to_string(&provider).map_or_else(
                        |e| {
                            console_error!("failed to encode provider: {e}");
                            Response::error("Internal error", 500)
                        },
                        |r| Response::from_json(&r),
                    )
                })
                .map_or_else(|e| e, |r| r)
        })
        .map_or_else(
            |e| match e {
                DatabaseError::TransactionError(e) => {
                    console_error!("failed to create credential: {e}");
                    Response::error("Internal error", 500)
                }
                _ => Response::error("Unknown error", 500),
            },
            |_| Response::ok(""),
        )
}

pub async fn login<TRepository: TaskRepository, CRepository: CredentialRepository>(
    mut request: Request,
    service: &Service<TRepository, CRepository>,
    token_sugar: &str,
) -> worker::Result<Response> {
    let data: Credential = request.json().await?;
    service
        .get_credential(data.username())
        .await
        .map(|credential| {
            if credential.password() == data.password() {
                let expr_hour = 6;
                let expr =
                    (OffsetDateTime::now_utc() + Duration::hours(expr_hour)).unix_timestamp();
                create_token(token_sugar, &data.username(), expr_hour)
                    .map_err(|e| {
                        console_error!("failed to create token: {e}");
                        Response::error("Internal error", 500)
                    })
                    .map(|token| TokenProvider { token, expr })
                    .map(|provider| {
                        serde_json::to_string(&provider).map_or_else(
                            |e| {
                                console_error!("failed to encode provider: {e}");
                                Response::error("Internal error", 500)
                            },
                            |r| Response::from_json(&r),
                        )
                    })
                    .map_or_else(|e| e, |r| r)
            } else {
                Response::error("Bad request", 400)
            }
        })
        .map_or_else(
            |e| match e {
                DatabaseError::TransactionError(e) => {
                    console_error!("failed to get credential: {e}");
                    Response::error("Internal error", 500)
                }
                DatabaseError::NotFound(_) => Response::error("not found", 404),
            },
            |r| r,
        )
}
