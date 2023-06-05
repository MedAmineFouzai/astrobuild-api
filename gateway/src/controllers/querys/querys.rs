use crate::{
    controllers::{
        schema::{
            CategoiresIds, CategoryOutput, CountryPrefixModel, FeatureOutput, ProjectOutput,
            SerlizedId, TemplateOutput, TemplateProtoTypeOutput, UserId, UserOutput,
        },
        MyToken,
    },constant::get_static_country_code,
    middleware::error::UserCustomResponseError,
};
use actix_web::http::StatusCode;
use async_graphql::ErrorExtensions;
use async_graphql::*;
use reqwest::header;

use load_dotenv::load_dotenv;
use std::env;

load_dotenv!();


pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_all_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserOutput>> {
        println!(
            "token :{:?}",
            &ctx.data_opt::<MyToken>()
                .map(|token| token.0.as_str())
                .unwrap_or("Authorization ")
        );
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!("{}/api/v1/users/all", env!("AUTH_URL")))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<UserOutput>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Users Not Found !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: String) -> FieldResult<UserOutput> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .post(&format!("{}/api/v1/users/get", env!("AUTH_URL")))
            .json(&UserId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserOutput = res.json::<UserOutput>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Not Found !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<CategoryOutput> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .post(&format!(
                "{}/api/v1/builder/category/get",
                env!("BUILDER_URL")
            ))
            .json(&SerlizedId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let category: CategoryOutput = res.json::<CategoryOutput>().await.unwrap();
                Ok(category)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", " Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_feature_by_id(&self, ctx: &Context<'_>, id: String) -> FieldResult<FeatureOutput> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .post(&format!(
                "{}/api/v1/builder/feature/get",
                env!("BUILDER_URL")
            ))
            .json(&SerlizedId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureOutput = res.json::<FeatureOutput>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", " Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }
    async fn get_prototype_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<TemplateProtoTypeOutput> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: SerlizedId = SerlizedId { id: id };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/prototype/get",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let prototype: TemplateProtoTypeOutput =
                    res.json::<TemplateProtoTypeOutput>().await.unwrap();
                Ok(prototype)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "ProtoType Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_template_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<TemplateOutput> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let res = client
            .post(&format!(
                "{}/api/v1/builder/template/get",
                env!("BUILDER_URL")
            ))
            .json(&SerlizedId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateOutput = res.json::<TemplateOutput>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", " Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_project_by_id(&self, ctx: &Context<'_>, id: String) -> FieldResult<ProjectOutput> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: SerlizedId = SerlizedId { id: id };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/project/get",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let project: ProjectOutput = res.json::<ProjectOutput>().await.unwrap();
                Ok(project)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Project Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_projects_by_client_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<Vec<ProjectOutput>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: SerlizedId = SerlizedId { id: id };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/project/user/get",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let project: Vec<ProjectOutput> = res.json::<Vec<ProjectOutput>>().await.unwrap();
                Ok(project)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", " User Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_projects(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<ProjectOutput>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
     

        let res = client
            .get(&format!(
                "{}/api/v1/builder/project/all",
                env!("BUILDER_URL")
            ))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let project: Vec<ProjectOutput> = res.json::<Vec<ProjectOutput>>().await.unwrap();
                Ok(project)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", " Projects Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_categories(&self, ctx: &Context<'_>) -> FieldResult<Vec<CategoryOutput>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!(
                "{}/api/v1/builder/category/all",
                env!("BUILDER_URL")
            ))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<CategoryOutput>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Categories not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_features(&self, ctx: &Context<'_>) -> FieldResult<Vec<FeatureOutput>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!(
                "{}/api/v1/builder/feature/all",
                env!("BUILDER_URL")
            ))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<FeatureOutput>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Features not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_templates(&self, ctx: &Context<'_>) -> FieldResult<Vec<TemplateOutput>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!(
                "{}/api/v1/builder/template/all",
                env!("BUILDER_URL")
            ))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<TemplateOutput>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Templates not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_templates_by_categories_id(
        &self,
        ctx: &Context<'_>,
        categories: Vec<String>,
    ) -> FieldResult<Vec<TemplateOutput>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let data: CategoiresIds = CategoiresIds {
            categories_id: categories,
        };
        let res = client
            .post(&format!(
                "{}/api/v1/builder/template/categories/all",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<TemplateOutput>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Templates Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_country_code(&self) -> FieldResult<Vec<CountryPrefixModel>> {
        Ok(get_static_country_code()
            .into_iter()
            .map(|value| CountryPrefixModel {
                country: value.0.to_string(),
                prefix: value.1.to_string(),
            })
            .collect::<Vec<CountryPrefixModel>>())
    }

    // async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
    //     ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    // }
}
