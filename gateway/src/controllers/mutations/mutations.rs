use crate::{
    controllers::schema::{Project, SendAccountModel, TemplateProtoType,  AddressOutput,  Category, CategoryInput, CategoryOutput,
        Connections,  DeleteUserById, EmailModel, Feature, FeatureInput,
        FeatureOutput, FeatureToAnyModel, File, FileWithOutOId, PasswordInput, PasswordModel,
         PhoneOutput, ProtoType, Relations, SerlizedId, SpecificationInput,
        SpecificationOutput, Template, TemplateDefactoredOutput, TemplateInput, TemplateOutput,
        TemplateProtoTypeInput, TemplateProtoTypeOutput, TemplateUpdateInput, UpdateFeatureWireframes,
        UpdateUserInfo, UpdateUserInput, UpdateUserPassword, UserAuthenticationOutput, 
        UserInfo, UserInput, UserLoginModel, UserModel, UserOutput, DelivrableOutput, DevtimeOutput, PaymentOptionOutput, ProjectFullBuild, ProjectInput,
        ProjectOutput, ProjectProposal, ProjectState, ProjectUpdateModel, ProposalInput,InputFile,
        ProposalOutput, ResourceOutput, State,ProjectFile,ProjectFileInput,ProjectFullBuildInput},  MyToken,
    middleware::error::UserCustomResponseError,
};
use actix_web::http::StatusCode;
use async_graphql::ErrorExtensions;
use async_graphql::*;
use bson::oid::ObjectId;
use reqwest::header;
use std::env;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn signup(
        &self,
        email: String,
        password: String,
    ) -> FieldResult<UserAuthenticationOutput> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/signup", env!("AUTH_URL")))
            .json(&UserModel {
                email: email,
                password: password,
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: PhoneOutput {
                    prefix: "".to_string(),
                    number: "".to_string(),
                },
                address: AddressOutput {
                    place: "".to_string(),
                    city: "".to_string(),
                    zip: "".to_string(),
                    country: "".to_string(),
                },

                role: "Client".to_string(),
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserAuthenticationOutput =
                    res.json::<UserAuthenticationOutput>().await.unwrap();
                Ok(user)
            }
            StatusCode::CONFLICT => Err(UserCustomResponseError::Conflict
                .extend_with(|_, e| e.set("info", "User Already Exist !"))),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials Or User Dosent Existe !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn create_user(&self, user: UserInput) -> FieldResult<UserOutput> {
        let email_model = &SendAccountModel {
            email: user.email.clone(),
            password: user.password.clone(),
            role:user.role.clone(),
        };
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/signup", env!("AUTH_URL")))
            .json(&UserModel {
                email: user.email,
                password: user.password,
                first_name: user.first_name,
                last_name: user.last_name,
                phone: PhoneOutput {
                    prefix: user.phone.prefix,
                    number: user.phone.number,
                },
                address: AddressOutput {
                    place: user.address.place,
                    city: user.address.city,
                    zip: user.address.zip,
                    country: user.address.country,
                },

                role:user.role,
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let client = reqwest::Client::new();
                let res = client
                    .post(&format!(
                        "{}/api/v1/users/auth/send/account",
                        env!("AUTH_URL")
                    ))
                    .json(email_model)
                    .send()
                    .await
                    .unwrap();
                match res.status() {
                    StatusCode::OK => {
                        let user: UserOutput = res.json::<UserOutput>().await.unwrap();
                        Ok(user)
                    }
                    _ => Err(UserCustomResponseError::ServerError
                        .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
                }
            }
            StatusCode::CONFLICT => Err(UserCustomResponseError::Conflict
                .extend_with(|_, e| e.set("info", "User Already Exist !"))),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials Or User Dosent Existe !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn login(
        &self,
        email: String,
        password: String,
    ) -> FieldResult<UserAuthenticationOutput> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/login", env!("AUTH_URL")))
            .json(&UserLoginModel {
                email: email,
                password: password,
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserAuthenticationOutput =
                    res.json::<UserAuthenticationOutput>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        password: String,
    ) -> FieldResult<UserOutput> {
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
        let data: DeleteUserById = DeleteUserById {
            id: id,
            password: password,
        };
        let res = client
            .delete(&format!("{}/api/v1/users/delete", env!("AUTH_URL")))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserOutput = res.json::<UserOutput>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound.extend_with(|_, e| {
                e.set("info", "User Not Found !")
            })),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User Not ALLowed  !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_user_info(
        &self,
        ctx: &Context<'_>,
        user: UpdateUserInput,
    ) -> FieldResult<UserOutput> {
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
            .put(&format!("{}/api/v1/users/update/info", env!("AUTH_URL")))
            .json(&UpdateUserInfo {
                id: user.id,
                user_info: UserInfo {
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                    phone: PhoneOutput {
                        prefix: user.phone.prefix,
                        number: user.phone.number,
                    },
                    address: AddressOutput {
                        place: user.address.place,
                        city: user.address.city,
                        zip: user.address.zip,
                        country: user.address.country,
                    },
                },
            })
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
                    e.set("info", "User Not ALLowed  !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_user_password(
        &self,
        ctx: &Context<'_>,
        id: String,
        password: PasswordInput,
    ) -> FieldResult<UserOutput> {
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
            .put(&format!(
                "{}/api/v1/users/update/password",
                env!("AUTH_URL")
            ))
            .json(&UpdateUserPassword {
                id: id,
                set_password: PasswordModel {
                    old_password: password.old_password,
                    new_password: password.new_password,
                },
            })
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
                    e.set("info", "User Not ALLowed  !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn reset_user_password(
        &self,
        // ctx: &Context<'_>,
        email: String,
    ) -> FieldResult<UserOutput> {
        let client = reqwest::Client::new();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/users/auth/reset", env!("AUTH_URL")))
            .json(&EmailModel { email: email })
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
                    e.set("info", "User Not ALLowed!")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn confirm_user_reset_password(
        &self,
        // ctx: &Context<'_>,
        id: String,
        password: String,
    ) -> FieldResult<UserOutput> {
        let client = reqwest::Client::new();
        // let data = ;
        let res = client
            .put(&format!(
                "{}/api/v1/users/auth/reset/confirm",
                env!("AUTH_URL")
            ))
            .json(&UpdateUserPassword {
                id: id,
                set_password: PasswordModel {
                    old_password: "".to_string(),
                    new_password: password,
                },
            })
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
                    e.set("info", "User Not ALLowed !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    //Categoru Mutations
    async fn delete_category(&self, ctx: &Context<'_>, id: String) -> FieldResult<CategoryOutput> {
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
            .delete(&format!(
                "{}/api/v1/builder/category/delete",
                env!("BUILDER_URL")
            ))
            .json(&data)
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
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_feature(&self, ctx: &Context<'_>, id: String) -> FieldResult<FeatureOutput> {
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
            .delete(&format!(
                "{}/api/v1/builder/feature/delete",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureOutput = res.json::<FeatureOutput>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_template(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<TemplateDefactoredOutput> {
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
            .delete(&format!(
                "{}/api/v1/builder/template/delete",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateDefactoredOutput =
                    res.json::<TemplateDefactoredOutput>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_template_features(
        &self,
        ctx: &Context<'_>,
        id: String,
        features_id: Vec<String>,
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
        let data: FeatureToAnyModel = FeatureToAnyModel {
            id: id,
            features_id: features_id,
        };
        let res = client
            .put(&format!(
                "{}/api/v1/builder/template/feature/update",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateOutput = res.json::<TemplateOutput>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    // async fn delete_template_feature(
    //     &self,
    //     ctx: &Context<'_>,
    //     id: String,
    //     featuer_id: String,
    // ) -> FieldResult<TemplateResponseModel> {
    //     let mut headers = header::HeaderMap::new();
    //     headers.insert(
    //         header::AUTHORIZATION,
    //         header::HeaderValue::from_str(
    //             &ctx.data_opt::<MyToken>()
    //                 .map(|token| token.0.as_str())
    //                 .unwrap_or("Authorization "),
    //         )
    //         .unwrap(),
    //     );
    //     let client = reqwest::Client::builder()
    //         .default_headers(headers)
    //         .build()
    //         .unwrap();
    //     let data: FeatureToAnyModel = FeatureToAnyModel {
    //         id: id,
    //         features_id: vec![featuer_id],
    //     };
    //     let res = client
    //         .delete(&format!(
    //             "{}/api/v1/builder/template/feature/delete",
    //             env!("BASE_URL")
    //         ))
    //         .json(&data)
    //         .send()
    //         .await
    //         .unwrap();

    //     match res.status() {
    //         StatusCode::OK => {
    //             let template: TemplateResponseModel =
    //                 res.json::<TemplateResponseModel>().await.unwrap();
    //             Ok(template)
    //         }

    //         StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
    //             .extend_with(|_, e| e.set("info", "Category Dosent Existe To Delete !"))),
    //         StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
    //             .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
    //         _ => Err(UserCustomResponseError::ServerError
    //             .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
    //     }
    // }

    async fn add_template_specification(
        &self,
        ctx: &Context<'_>,
        id: String,
        specification: SpecificationInput,
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

        let form = reqwest::multipart::Form::new()
            .text("purpose", specification.introduction.purpose)
            .text(
                "document_conventions",
                specification.introduction.document_conventions,
            )
            .text(
                "intended_audience",
                specification.introduction.intended_audience,
            )
            .text("project_scope", specification.introduction.project_scope)
            .text("perspective", specification.overall_description.perspective)
            .text(
                "user_characteristics",
                specification.overall_description.user_characteristics,
            )
            .text(
                "operating_environment",
                specification.overall_description.operating_environment,
            )
            .text(
                "design_implementation_constraints",
                specification
                    .overall_description
                    .design_implementation_constraints,
            )
            .text(
                "user_documentation",
                specification.overall_description.user_documentation,
            )
            .text(
                "assemptions_dependencies",
                specification.overall_description.assemptions_dependencies,
            )
            .text(
                "performance_requirements",
                specification
                    .non_functional_requirements
                    .performance_requirements,
            )
            .text(
                "safety_requirements",
                specification
                    .non_functional_requirements
                    .safety_requirements,
            )
            .text(
                "security_requirements",
                specification
                    .non_functional_requirements
                    .security_requirements,
            )
            .text(
                "software_quality_attributes",
                specification
                    .non_functional_requirements
                    .software_quality_attributes,
            )
            .text("other_requirements", specification.other_requirements)
            .text("glossary", specification.glossary)
            .text("analysis_models", specification.analysis_models)
            .text("issues_list", specification.issues_list)
            .text("id", id);

        let res = client
            .put(&format!(
                "{}/api/v1/builder/template/specification/add",
                env!("BUILDER_URL")
            ))
            .multipart(form)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateOutput = res.json::<TemplateOutput>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_category(
        &self,
        ctx: &Context<'_>,
        category: CategoryInput,
    ) -> FieldResult<CategoryOutput> {
        // let mut file = category.image.value(ctx).unwrap();
        // let mut buffer = Vec::new();
        // file.content.read_to_end(&mut buffer).unwrap();
        // let encoded_file = encode(buffer);

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
        let data: Category = Category {
            name: category.name,
            description: category.description,
            image: File {
                name: category.image.name,
                src: category.image.src,
            },
        };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/category/create",
                env!("BUILDER_URL")
            ))
            .json(&data)
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
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_category(
        &self,
        ctx: &Context<'_>,
        id: String,
        category: CategoryInput,
    ) -> FieldResult<CategoryOutput> {
        // let mut file = category.image.value(ctx).unwrap();
        // let mut buffer = Vec::new();
        // file.content.read_to_end(&mut buffer).unwrap();
        // let encoded_file = encode(buffer);

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
        let data: CategoryOutput = CategoryOutput {
            id: id,
            name: category.name,
            description: category.description,
            image: File {
                name: category.image.name,
                src: category.image.src,
            },
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/category/update",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let category: CategoryOutput = res.json::<CategoryOutput>().await.unwrap();
                Ok(category)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_feature(
        &self,
        ctx: &Context<'_>,
        feature: FeatureInput,
    ) -> FieldResult<FeatureOutput> {
        // let mut image = feature.image.value(ctx).unwrap();
        // let mut buffer = Vec::new();
        // image.content.read_to_end(&mut buffer).unwrap();
        // let encoded_image = encode(buffer);
        // let mut wireframes = Vec::new();

        let wireframes = match feature.wireframes {
            Some(wireframes) => wireframes
                .into_iter()
                .map(|wireframe| {
                    // let mut wireframe = wireframe.value(ctx).unwrap();
                    // let mut buffer = Vec::new();
                    // wireframe.content.read_to_end(&mut buffer).unwrap();
                    // let encoded_image = encode(buffer);

                    FileWithOutOId {
                        id: ObjectId::new().to_string(),
                        name: wireframe.name,
                        src: wireframe.src,
                    }
                })
                .collect::<Vec<FileWithOutOId>>(),
            None => vec![],
        };

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
        let data: Feature = Feature {
            name: feature.name,
            description: feature.description,
            feature_type: feature.feature_type,
            image: File {
                name: feature.image.name,
                src:feature.image.src,
            },
            wireframes: Some(wireframes),
            price: feature.price,
            repo: feature.repo,
        };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/feature/create",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureOutput = res.json::<FeatureOutput>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_feature(
        &self,
        ctx: &Context<'_>,
        id: String,
        feature: FeatureInput,
    ) -> FieldResult<FeatureOutput> {
        // let mut image = feature.image.value(ctx).unwrap();
        // let mut buffer = Vec::new();
        // image.content.read_to_end(&mut buffer).unwrap();
        // let encoded_image = encode(buffer);
        // let mut wireframes = Vec::new();

        let wireframes = match feature.wireframes {
            Some(wireframes) => wireframes
                .into_iter()
                .map(|wireframe| {
                    // let mut wireframe = wireframe.value(ctx).unwrap();
                    // let mut buffer = Vec::new();
                    // wireframe.content.read_to_end(&mut buffer).unwrap();
                    // let encoded_image = encode(buffer);

                    FileWithOutOId {
                        id: ObjectId::new().to_string(),
                        name: wireframe.name,
                        src: wireframe.src,
                    }
                })
                .collect::<Vec<FileWithOutOId>>(),
            None => vec![],
        };

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
        let data: FeatureOutput = FeatureOutput {
            id: id,
            name: feature.name,
            description: feature.description,
            feature_type: feature.feature_type,
            image: File {
                name:feature.image.name,
                src: feature.image.src,
            },
            wireframes: Some(wireframes),
            price: feature.price,
            repo: feature.repo,
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/feature/update",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureOutput = res.json::<FeatureOutput>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_feature_wireframe(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<FeatureOutput> {
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
            .delete(&format!(
                "{}/api/v1/builder/feature/wireframe/delete",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureOutput = res.json::<FeatureOutput>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Wireframe Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_feature_wireframes(
        &self,
        ctx: &Context<'_>,
        id: String,
        wireframes: Vec<InputFile>,
    ) -> FieldResult<FeatureOutput> {
        let mut wireframes_info: Vec<FileWithOutOId> = Vec::new();
        for wireframe in wireframes {
            // let mut frame = wireframe.value(ctx).unwrap();
            // let mut buffer = Vec::new();
            // frame.content.read_to_end(&mut buffer).unwrap();
            // let encoded_image = encode(buffer);
            wireframes_info.push(FileWithOutOId {
                id: ObjectId::new().to_string(),
                name: wireframe.name,
                src: wireframe.src
            });
        }

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
        let data: UpdateFeatureWireframes = UpdateFeatureWireframes {
            id: id,
            wireframes: wireframes_info,
        };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/feature/wireframe/add",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureOutput = res.json::<FeatureOutput>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_template(
        &self,
        ctx: &Context<'_>,
        template: TemplateInput,
    ) -> FieldResult<TemplateOutput> {
        // let mut image: UploadValue = template.image.value(ctx).unwrap();
        // let mut buffer: Vec<u8> = Vec::new();
        // image.content.read_to_end(&mut buffer).unwrap();
        // let encoded_image: String = encode(buffer);

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
        let data: Template = Template {
            name: template.name,
            description: template.description,
            category: template.category,
            features: template.features,
            image: File {
                name: template.image.name,
                src: template.image.src,
            },
            specification:match template.specification {
                Some(specification) => Some(SpecificationOutput::build(specification)),
                None => Some(SpecificationOutput::new())
            } 
            ,
        };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/template/create",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateOutput = res.json::<TemplateOutput>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_template(
        &self,
        ctx: &Context<'_>,
        id: String,
        template: TemplateUpdateInput,
        specification: Option<SpecificationInput>,
    ) -> FieldResult<TemplateOutput> {
        // let mut image: UploadValue = template.image.value(ctx).unwrap();
        // let mut buffer: Vec<u8> = Vec::new();
        // image.content.read_to_end(&mut buffer).unwrap();
        // let encoded_image: String = encode(buffer);

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
        let data: TemplateDefactoredOutput = TemplateDefactoredOutput {
            id: id.clone(),
            name: template.name,
            description: template.description,
            category: template.category,
            features:match template.features {
                Some(featuers)=>Some(featuers),
                None=>Some(vec![])
            },
            image: File {
                name: template.image.name,
                src: template.image.src,
            },
            specification: match specification{
                Some(specification)=>Some(SpecificationOutput::build(specification)),
                None=>Some(SpecificationOutput::new())
            },
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/template/update",
                env!("BUILDER_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                Ok(res.json::<TemplateOutput>().await.unwrap())
                // match specification {
                //     Some(specification) => {
                //         self.add_template_specification(ctx, id.clone(), specification)
                //             .await
                //     }
                //     None => Ok(res.json::<TemplateOutput>().await.unwrap()),
                // }
                // let template: TemplateResponseModel =

                // Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_prototype(
        &self,
        ctx: &Context<'_>,
        prototype: TemplateProtoTypeInput,
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
        let data: TemplateProtoType = TemplateProtoType {
            template_id: prototype.template_id,
            prototype: prototype
                .prototype
                .into_iter()
                .map(|prototype_stream| ProtoType {
                    feature_id: prototype_stream.feature_id,
                    connections: prototype_stream
                        .connections
                        .into_iter()
                        .map(|connections_strem| Connections {
                            to: connections_strem.to,
                            releations: Relations {
                                back: connections_strem.releations.back,
                                forword: connections_strem.releations.forword,
                            },
                        })
                        .collect::<Vec<Connections>>(),
                })
                .collect::<Vec<ProtoType>>(),
        };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/prototype/add",
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
                .extend_with(|_, e| e.set("info", "PrototType Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_prototype(
        &self,
        ctx: &Context<'_>,
        prototype: TemplateProtoTypeInput,
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
        let data: TemplateProtoType = TemplateProtoType {
            template_id: prototype.template_id,
            prototype: prototype
                .prototype
                .into_iter()
                .map(|prototype_stream| ProtoType {
                    feature_id: prototype_stream.feature_id,
                    connections: prototype_stream
                        .connections
                        .into_iter()
                        .map(|connections_strem| Connections {
                            to: connections_strem.to,
                            releations: Relations {
                                back: connections_strem.releations.back,
                                forword: connections_strem.releations.forword,
                            },
                        })
                        .collect::<Vec<Connections>>(),
                })
                .collect::<Vec<ProtoType>>(),
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/prototype/update",
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
                .extend_with(|_, e| e.set("info", "ProtoType Not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_project(
        &self,
        ctx: &Context<'_>,
        project: ProjectInput,
    ) -> FieldResult<ProjectOutput> {
        // let mut image: UploadValue = project.image.value(ctx).unwrap();
        // let mut buffer: Vec<u8> = Vec::new();
        // image.content.read_to_end(&mut buffer).unwrap();
        // let encoded_image: String = encode(buffer);
        // println!("image file :{:?}", image.filename);
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
        let data: Project = Project {
            client_id: project.client_id,
            name: project.name,
            image: File {
                name: project.image.name,
                src: project.image.src,
            },
            platforms: project.platforms,
            template: project.template,
            features: project.features,
            state: "OnReview".to_string(),
            proposal: Some(ProposalOutput {
                devtime: DevtimeOutput {
                    months: 0,
                    days: 0,
                    hours: 0,
                },
                summary: "".to_string(),
                purpose: "".to_string(),
                resources: vec![],
            }),
            payment_option: PaymentOptionOutput {
                opt_one: project.payment_option.opt_one,
                opt_two: project.payment_option.opt_two,
                opt_three: project.payment_option.opt_three,
            },
            delivrable: Some(DelivrableOutput {
                specification: File {
                    name: "".to_string(),
                    src: "".to_string(),
                },
                full_build: "".to_string(),
                mvp: File {
                    name: "".to_string(),
                    src: "".to_string(),
                },
                design: File {
                    name: "".to_string(),
                    src: "".to_string(),
                },
            }),
            total_price: project.total_price,
        };

        let res = client
            .post(&format!(
                "{}/api/v1/builder/project/add",
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

    async fn change_project_state(
        &self,
        ctx: &Context<'_>,
        id: String,
        state: State,
    ) -> FieldResult<ProjectOutput> {
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
        let data: ProjectState = ProjectState {
            id: id,
            state: match state {
                State::Approved => "Approved".to_string(),
                State::Declined => "Declined".to_string(),
                State::OnReview => "OnReview".to_string(),
                State::Archived => "Archived".to_string(),
            },
        };

        let res = client
            .delete(&format!(
                "{}/api/v1/builder/project/state",
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
                .extend_with(|_, e| e.set("info", "Project Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: String,
        image: InputFile,
    ) -> FieldResult<ProjectOutput> {
        // let mut project_image: UploadValue = image.value(ctx).unwrap();
        // let mut buffer: Vec<u8> = Vec::new();
        // project_image.content.read_to_end(&mut buffer).unwrap();
        // let encoded_image: String = encode(buffer);
        // println!("image file :{:?}", project_image.filename);

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
        let data: ProjectUpdateModel = ProjectUpdateModel {
            id: id,
            name: name,
            image: File {
                name: image.name,
                src: image.src,
            },
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/project/update",
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
                .extend_with(|_, e| e.set("info", "Project Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_project_proposal(
        &self,
        ctx: &Context<'_>,
        id: String,
        proposal: ProposalInput,
    ) -> FieldResult<ProjectOutput> {
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
        let data: ProjectProposal = ProjectProposal {
            id: id,
            proposal: ProposalOutput {
                devtime: DevtimeOutput {
                    months: proposal.devtime.months,
                    days: proposal.devtime.days,
                    hours: proposal.devtime.hours,
                },
                summary: proposal.summary,
                purpose: proposal.purpose,
                resources: proposal
                    .resources
                    .into_iter()
                    .map(|resource| ResourceOutput {
                        resource_type: resource.resource_type,
                        developers: resource.developers,
                    })
                    .collect::<Vec<ResourceOutput>>(),
            },
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/project/proposal/add",
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
                .extend_with(|_, e| e.set("info", "Project Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_project_mvp(
        &self,
        ctx: &Context<'_>,
        mvp: ProjectFileInput,
    ) -> FieldResult<ProjectOutput> {
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
        let data: ProjectFile=ProjectFile{
            id:mvp.id,
            name:mvp.name,
            src:mvp.src
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/project/mvp/add",
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
                .extend_with(|_, e| e.set("info", "Project Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }
               
    async fn add_project_full_build(
        &self,
        ctx: &Context<'_>,
        full_build: ProjectFullBuildInput,
    ) -> FieldResult<ProjectOutput> {
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
        let data: ProjectFullBuild=ProjectFullBuild{
            id: full_build.id,
            url: full_build.url,
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/project/full_build/add",
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
                .extend_with(|_, e| e.set("info", "Project Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

      async fn add_project_design(
        &self,
        ctx: &Context<'_>,
        design: ProjectFileInput,
    ) -> FieldResult<ProjectOutput> {
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
        let data: ProjectFile=ProjectFile{
            id:design.id,
            name:design.name,
            src:design.src
        };

        let res = client
            .put(&format!(
                "{}/api/v1/builder/project/design/add",
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
                .extend_with(|_, e| e.set("info", "Project Not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    } 
           
        // match res.status() {
        //     StatusCode::OK => {
        //         let project: ProjectOutput = res.json::<ProjectOutput>().await.unwrap();
        //         Ok(project)
        //     }
        //     StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
        //         .extend_with(|_, e| e.set("info", "Project Deliverables Cant Be Created!"))),
        //     StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
        //         .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
        //     _ => Err(UserCustomResponseError::ServerError
        //         .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        // }
}
