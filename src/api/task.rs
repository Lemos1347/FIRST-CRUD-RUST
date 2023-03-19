use crate::db::{
    user::{self},
    PrismaClient,
};
use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, web::Query, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ParamsGetUserByEmail {
   email: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateResponse {
   status: String,
   message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserUpdate {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[post("/create/user")]
pub async fn create_user(body: Json<UserInfo>, client: Data<PrismaClient>) -> Json<CreateResponse> {
    let _user = client
        .user()
        .create(
            body.name.clone(),
            body.email.clone(),
            body.password.clone(),
            vec![],
        )
        .exec()
        .await
        .unwrap();
    let create_response = CreateResponse {
        status: "success".to_string(),
        message: "User created with sucess!".to_string(),
    };
    return Json(create_response);
}

#[get("/user/{id}")]
pub async fn get_user_by_id(path: Path<String>, client: Data<PrismaClient>) -> impl Responder {
    let user = client
        .user()
        .find_unique(user::id::equals(path.clone()))
        .exec()
        .await
        .unwrap();

    return HttpResponse::Ok().json(user);
}

#[get("/users")]
pub async fn get_users(client: Data<PrismaClient>) -> impl Responder {
    let user = client.user().find_many(vec![]).exec().await.unwrap();

    return HttpResponse::Ok().json(user);
}

#[get("/user_email")]
pub async fn get_user_by_email(
    query: Query<ParamsGetUserByEmail>,
    client: Data<PrismaClient>,
) -> impl Responder {
    print!("Chegou");
    let user = client
        .user()
        .find_first(vec![user::email::equals(query.email.clone())])
        .exec()
        .await
        .unwrap();

    return HttpResponse::Ok().json(user);
}

#[put("/user/{id}")]
pub async fn edit_user_by_id(
    path: Path<String>,
    body: Json<UserUpdate>,
    client: Data<PrismaClient>,
) -> Json<CreateResponse> {
    let mut updated: String = "".to_string();
    if let Some(name) = &body.name {
        let _ = client
            .user()
            .update(
                user::id::equals(path.clone()),
                vec![user::name::set(name.clone())],
            )
            .exec()
            .await
            .unwrap();
        updated += &" name".to_string()
    }

    if let Some(email) = &body.email {
        let _ = client
            .user()
            .update(
                user::id::equals(path.clone()),
                vec![user::name::set(email.clone())],
            )
            .exec()
            .await
            .unwrap();
        updated += &" email".to_string()
    }

    if let Some(password) = &body.password {
        let _ = client
            .user()
            .update(
                user::id::equals(path.clone()),
                vec![user::name::set(password.clone())],
            )
            .exec()
            .await
            .unwrap();
        updated += &" password".to_string()
    }

    let create_response = CreateResponse {
        status: "success".to_string(),
        message: format!(
            "The following fields [{}] was updated with sucess!",
            updated
        ),
    };
    return Json(create_response);
}

#[delete("/user/{id}")]
pub async fn delete_user_by_id(
    path: Path<String>,
    client: Data<PrismaClient>,
) -> Json<CreateResponse> {
    let _user = client
        .user()
        .delete(user::id::equals(path.clone()))
        .exec()
        .await
        .unwrap();

    let create_response = CreateResponse {
        status: "success".to_string(),
        message: "User deleted with sucess!".to_string(),
    };
    return Json(create_response);
}
