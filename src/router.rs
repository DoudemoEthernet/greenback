use worker::{console_error, Headers, Request, Response, RouteContext};

use crate::{
    db::{
        entity::{account::AccountId, task::TaskId},
        error::DatabaseError,
        repository::task::TaskRepository,
        service::Service,
    },
    util::task::{PatchTask, PostTask, ResponseTask},
};

pub trait BasicHeader {
    fn append_header(self) -> Self;
}

impl BasicHeader for worker::Result<Response> {
    fn append_header(self) -> worker::Result<Response> {
        let mut headers = Headers::new();
        headers.append("Access-Control-Allow-Origin", "*")?;
        headers.append("Access-Control-Allow-Methods", "POST, PATCH, DLETE")?;
        self.map(|res| res.with_headers(headers))
    }
}

pub async fn post_task<Repository: TaskRepository>(
    mut request: Request,
    service: &Service<Repository>,
) -> worker::Result<Response> {
    let data: PostTask = request.json().await?;
    service
        .create_task(data)
        .await
        .map_or_else(
            |e| match e {
                DatabaseError::TransactionError(e) => {
                    let message = "Failed to post task";
                    console_error!("{message}: {e}");
                    Response::error(message, 500)
                }
                _ => Response::error("unknown error", 500),
            },
            |_| Response::ok(""),
        )
        .append_header()
}

pub async fn get_task<Repository: TaskRepository>(
    service: &Service<Repository>,
) -> worker::Result<Response> {
    service
        .get_all_tasks(AccountId::new(0))
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
        .map_or_else(|e| e, |r| r)
        .append_header()
}

pub async fn patch_task<Repository: TaskRepository>(
    mut request: Request,
    service: &Service<Repository>,
) -> worker::Result<Response> {
    let data: PatchTask = request.json().await?;
    service
        .update_task(data)
        .await
        .map_or_else(
            |e| match e {
                DatabaseError::TransactionError(e) => {
                    console_error!("faled to update task: {e}");
                    Response::error("Internal error", 500)
                }
                _ => Response::error("Unknown error", 500),
            },
            |_| Response::ok(""),
        )
        .append_header()
}

pub async fn delete_task<Repository: TaskRepository>(
    context: &RouteContext<()>,
    service: &Service<Repository>,
) -> worker::Result<Response> {
    if let Some(id) = context.param("id") {
        return match TaskId::try_from(id.as_str()) {
            Err(e) => {
                console_error!("faield to convert uuid: {e}");
                Response::error("Invalid id", 400)
            }
            Ok(id) => service
                .delete_task(id)
                .await
                .map_err(|e| match e {
                    DatabaseError::NotFound(_) => Response::error("target not found", 404),
                    DatabaseError::TransactionError(e) => {
                        console_error!("failed to delete task: {e}");
                        Response::error("Internal error", 500)
                    }
                })
                .map_or_else(|e| e, |_| Response::ok("success")),
        }
        .append_header();
    };
    Response::error("bad request", 400).append_header()
}
