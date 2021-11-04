use axum::{
    extract::{Extension, Form},
    http::HeaderMap,
    http::StatusCode,
    response::Html,
};

use crate::{
    db::{subject, topic},
    form,
    handler::{
        helper::{get_client, log_error, render},
        redirect::redirect,
    },
    html::backend::topic::AddTemplate,
    md,
    model::AppState,
    Result,
};
use std::sync::Arc;

pub async fn add(Extension(state): Extension<Arc<AppState>>) -> Result<Html<String>> {
    let handler_name = "backend_topic_add";
    let client = get_client(state, handler_name).await?;
    let subjects = subject::all(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = AddTemplate { subjects };
    render(tmpl, handler_name)
}

pub async fn add_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(ct): Form<form::CreateTopic>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_topic_add";
    let html_text = md::to_html(&ct.md);
    let mut client = get_client(state, handler_name).await?;
    topic::create(&mut client, &ct, &html_text)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/topic?msg=文章添加成功")
}
