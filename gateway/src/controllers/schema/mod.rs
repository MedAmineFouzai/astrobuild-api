use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct UserId {
    pub id: String,
}



#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct DeleteUserById {
    pub id: String,
    pub password: String,
}

#[derive(Debug, Serialize, SimpleObject, Deserialize, Clone)]
pub struct PhoneOutput {
    pub prefix: String,
    pub number: String,
}

#[derive(Debug, Serialize, InputObject, Deserialize, Clone)]
pub struct PhoneInput {
    pub prefix: String,
    pub number: String,
}

#[derive(Debug, Serialize, SimpleObject, Deserialize, Clone)]
pub struct CountryPrefixModel {
    pub country: String,
    pub prefix: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct AddressOutput {
    pub place: String,
    pub city: String,
    pub zip: String,
    pub country: String,
}

#[derive(Debug, Clone, InputObject, Deserialize, Serialize)]
pub struct AddressInput {
    pub place: String,
    pub city: String,
    pub zip: String,
    pub country: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct UserModel {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneOutput,
    pub address: AddressOutput,
    pub role: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct UserOutput {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneOutput,
    pub address: AddressOutput,
    pub role: String,
}

#[derive(InputObject)]
pub struct UserInput {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneInput,
    pub address: AddressInput,
    pub role: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneInput,
    pub address: AddressInput,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerlizedId {
    pub id: String,
}

#[derive(Debug, InputObject, Serialize, Deserialize, Clone)]
pub struct PasswordInput {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct PasswordModel {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: PhoneOutput,
    pub address: AddressOutput,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UpdateUserInfo {
    pub id: String,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendAccountModel {
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    pub id: String,
    pub set_password: PasswordModel,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UserLoginModel {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailModel {
    pub email: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct UserAuthenticationOutput {
    pub user: UserOutput,
    pub token: String,
}

/////////////////////////////////// Category model

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct CategoryOutput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: File,
}

#[derive(InputObject)]
pub struct CategoryInput {
    pub name: String,
    pub description: String,
    pub image: InputFile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub description: String,
    pub image: File,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoiresIds {
    pub categories_id: Vec<String>,
}

#[derive(Debug, SimpleObject, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub src: String,
}

#[derive(InputObject)]
pub struct InputFile {
    pub name:String,
    pub src:String,
}

/////////////////////////////////// Feature model

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct FeatureOutput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub feature_type: String,
    pub image: File,
    pub wireframes: Option<Vec<FileWithOutOId>>,
    pub price: f64,
    pub repo: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFeatureWireframes {
    pub id: String,
    pub wireframes: Vec<FileWithOutOId>,
}

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub feature_type: String,
    pub image: File,
    pub wireframes: Option<Vec<FileWithOutOId>>,
    pub price: f64,
    pub repo: String,
}

#[derive(InputObject)]
pub struct FeatureInput {
    pub name: String,
    pub description: String,
    pub feature_type: String,
    pub image: InputFile,
    pub wireframes: Option<Vec<InputFile>>,
    pub price: f64,
    pub repo: String,
}

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct FileWithOutOId {
    pub id: String,
    pub name: String,
    pub src: String,
}

/////////////////////////////////// Template model

#[derive(Debug, Serialize, SimpleObject, Clone, Deserialize)]
pub struct TemplateOutput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<FeatureOutput>>,
    pub image: File,
    pub specification: Option<SpecificationOutput>,
    // pub prototype_id: String,
}
#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct TemplateDefactoredOutput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<String>>,
    pub image: File,
    pub specification: Option<SpecificationOutput>,
    // pub prototype_id: String,
}

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<String>>,
    pub image: File,
    pub specification: Option<SpecificationOutput>,
    // pub prototype_id: String,
}

#[derive(InputObject)]
pub struct TemplateInput {
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<String>>,
    pub image: InputFile,
    pub specification: Option<SpecificationInput>,
}

#[derive(InputObject)]
pub struct TemplateUpdateInput {
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<String>>,
    pub image: InputFile,
  
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct SpecificationOutput {
    pub introduction: IntroductionOutput,
    pub overall_description: OverallDescriptionOutput,
    pub non_functional_requirements: NonFunctionalRequirementsOutput,
    pub other_requirements: String,
    pub glossary: String,
    pub analysis_models: String,
    pub issues_list: String,
}

impl SpecificationOutput {
    pub fn build(specification:SpecificationInput)->SpecificationOutput{
        SpecificationOutput {
            introduction: IntroductionOutput{
                purpose: specification.introduction.purpose,
                document_conventions: specification.introduction.document_conventions,
                intended_audience: specification.introduction.intended_audience,
                project_scope: specification.introduction.project_scope,
            },
            overall_description: OverallDescriptionOutput{
                perspective: specification.overall_description.perspective,
                user_characteristics: specification.overall_description.user_characteristics,
                operating_environment: specification.overall_description.operating_environment,
                design_implementation_constraints: specification.overall_description.design_implementation_constraints,
                user_documentation: specification.overall_description.user_documentation,
                assemptions_dependencies: specification.overall_description.assemptions_dependencies,
            },
            non_functional_requirements: NonFunctionalRequirementsOutput{
                performance_requirements: specification.non_functional_requirements.performance_requirements,
                safety_requirements: specification.non_functional_requirements.safety_requirements,
                security_requirements: specification.non_functional_requirements.security_requirements,
                software_quality_attributes: specification.non_functional_requirements.software_quality_attributes,
            },
            other_requirements: specification.other_requirements,
            glossary: specification.glossary,
            analysis_models: specification.analysis_models,
            issues_list: specification.issues_list,
        }
    }
}


#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct IntroductionOutput {
    pub purpose: String,
    pub document_conventions: String,
    pub intended_audience: String,
    pub project_scope: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct OverallDescriptionOutput {
    pub perspective: String,
    pub user_characteristics: String,
    pub operating_environment: String,
    pub design_implementation_constraints: String,
    pub user_documentation: String,
    pub assemptions_dependencies: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct NonFunctionalRequirementsOutput {
    pub performance_requirements: String,
    pub safety_requirements: String,
    pub security_requirements: String,
    pub software_quality_attributes: String,
}

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct FeatureToAnyModel {
    pub id: String,
    pub features_id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct SpecificationInput {
    pub introduction: IntroductionInput,
    pub overall_description: OverallDescriptionInput,
    pub non_functional_requirements: NonFunctionalRequirementsInput,
    pub other_requirements: String,
    pub glossary: String,
    pub analysis_models: String,
    pub issues_list: String,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct IntroductionInput {
    pub purpose: String,
    pub document_conventions: String,
    pub intended_audience: String,
    pub project_scope: String,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct OverallDescriptionInput {
    pub perspective: String,
    pub user_characteristics: String,
    pub operating_environment: String,
    pub design_implementation_constraints: String,
    pub user_documentation: String,
    pub assemptions_dependencies: String,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct NonFunctionalRequirementsInput {
    pub performance_requirements: String,
    pub safety_requirements: String,
    pub security_requirements: String,
    pub software_quality_attributes: String,
}

impl SpecificationOutput {
    pub fn new() -> SpecificationOutput {
        SpecificationOutput {
            introduction: IntroductionOutput {
                purpose: "".to_string(),
                document_conventions: "".to_string(),
                intended_audience: "".to_string(),
                project_scope: "".to_string(),
            },
            overall_description: OverallDescriptionOutput {
                perspective: "".to_string(),
                user_characteristics: "".to_string(),
                operating_environment: "".to_string(),
                design_implementation_constraints: "".to_string(),
                user_documentation: "".to_string(),
                assemptions_dependencies: "".to_string(),
            },
            non_functional_requirements: NonFunctionalRequirementsOutput {
                performance_requirements: "".to_string(),
                safety_requirements: "".to_string(),
                security_requirements: "".to_string(),
                software_quality_attributes: "".to_string(),
            },
            other_requirements: "".to_string(),
            glossary: "".to_string(),
            analysis_models: "".to_string(),
            issues_list: "".to_string(),
        }
    }
}

////////////////////////////// prottype model

//input
#[derive(InputObject)]
pub struct TemplateProtoTypeInput {
    pub template_id: String,
    pub prototype: Vec<ProtoTypeInput>,
}
#[derive(InputObject)]
pub struct ProtoTypeInput {
    pub feature_id: String,
    pub connections: Vec<ConnectionsInput>,
}
#[derive(InputObject)]
pub struct ConnectionsInput {
    pub to: String,
    pub releations: RelationsInput,
}
#[derive(InputObject)]
pub struct RelationsInput {
    pub back: bool,
    pub forword: bool,
}

//object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateProtoType {
    pub template_id: String,
    pub prototype: Vec<ProtoType>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoType {
    pub feature_id: String,
    pub connections: Vec<Connections>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connections {
    pub to: String,
    pub releations: Relations,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relations {
    pub back: bool,
    pub forword: bool,
}

//output

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct TemplateProtoTypeOutput {
    pub id: String,
    pub template: String,
    pub prototype: Vec<ProtoTypeOutput>,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct ProtoTypeOutput {
    pub feature: FeatureOutput,
    pub connections: Vec<ConnectionsOutput>,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct ConnectionsOutput {
    pub to: String,
    pub releations: RelationsOutput,
}

#[derive(Debug, SimpleObject, Clone, Serialize, Deserialize)]
pub struct RelationsOutput {
    pub back: bool,
    pub forword: bool,
}

///////////////////project

#[derive(InputObject)]
pub struct ProjectInput {
    pub client_id: String,
    pub name: String,
    pub image: InputFile,
    pub platforms: Vec<String>,
    pub template: String,
    pub features: Vec<String>,
    // pub state: String,
    // pub proposal: Option<ProposalInput>,
    pub payment_option: PaymentOptionInput,
    pub delivrable: Option<DelivrableInput>,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Project {
    pub client_id: String,
    pub name: String,
    pub image: File,
    pub platforms: Vec<String>,
    pub template: String,
    pub features: Vec<String>,
    pub state: String,
    pub proposal: Option<ProposalOutput>,
    pub payment_option: PaymentOptionOutput,
    pub delivrable: Option<DelivrableOutput>,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectUpdateModel {
    pub id: String,
    pub name: String,
    pub image: File,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectState {
    pub id: String,
    pub state: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum State {
    #[graphql(name = "Approved")]
    Approved,
    #[graphql(name = "Declined")]
    Declined,
    #[graphql(name = "OnReview")]
    OnReview,
    #[graphql(name = "Archived")]
    Archived,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectFullBuild {
    pub id: String,
    pub url: String,

}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectProposal {
    pub id: String,
    pub proposal: ProposalOutput,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct ProjectOutput {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub image: File,
    pub platforms: Vec<String>,
    pub template: TemplateOutput,
    pub features: Vec<FeatureOutput>,
    pub state: String,
    pub proposal: Option<ProposalOutput>,
    pub payment_option: PaymentOptionOutput,
    pub delivrable: Option<DelivrableOutput>,
    pub total_price: f64,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct ProposalOutput {
    pub devtime: DevtimeOutput,
    pub summary: String,
    pub purpose: String,
    pub resources: Vec<ResourceOutput>,
}

#[derive(InputObject)]
pub struct ProposalInput {
    pub devtime: DevtimeInput,
    pub summary: String,
    pub purpose: String,
    pub resources: Vec<ResourceInput>,
}

#[derive(InputObject)]
pub struct PaymentOptionInput {
    pub opt_one: i32,
    pub opt_two: i32,
    pub opt_three: i32,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct PaymentOptionOutput {
    pub opt_one: i32,
    pub opt_two: i32,
    pub opt_three: i32,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct DevtimeOutput {
    pub months: i32,
    pub days: i32,
    pub hours: i32,
}

#[derive(InputObject)]
pub struct DevtimeInput {
    pub months: i32,
    pub days: i32,
    pub hours: i32,
}
#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct ResourceOutput {
    pub resource_type: String,
    pub developers: i32,
}

#[derive(InputObject)]
pub struct ResourceInput {
    pub resource_type: String,
    pub developers: i32,
}

#[derive(Debug, SimpleObject, Serialize, Clone, Deserialize)]
pub struct DelivrableOutput {
    pub specification: File,
    pub full_build: String,
    pub mvp: File,
    pub design: File,
}

#[derive(InputObject)]
pub struct DelivrableInput {
    pub specification: bool,
    pub full_build: bool,
    pub mvp: bool,
    pub design: bool,
}


#[derive(InputObject)]
pub struct ProjectFileInput {
    pub id:String,
    pub name: String,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub id:String,
    pub name: String,
    pub src: String,
}

#[derive(InputObject)]
pub struct ProjectFullBuildInput {
    pub id:String,
    pub url: String,
}


