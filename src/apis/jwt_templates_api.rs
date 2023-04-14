/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::Error;
use crate::{apis::ResponseContent, clerk::Clerk};

/// struct for typed errors of method [`create_jwt_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateJwtTemplateError {
	Status400(crate::models::ClerkErrors),
	Status402(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_jwt_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteJwtTemplateError {
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jwt_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJwtTemplateError {
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_jwt_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListJwtTemplatesError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_jwt_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateJwtTemplateError {
	Status400(crate::models::ClerkErrors),
	Status402(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

pub struct JwtTemplate;

impl JwtTemplate {
	/// Create a new JWT template
	pub async fn create_jwt_template(
		clerk_client: &Clerk,
		create_jwt_template_request: Option<crate::models::CreateJwtTemplateRequest>,
	) -> Result<crate::models::JwtTemplate, Error<CreateJwtTemplateError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/jwt_templates", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&create_jwt_template_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateJwtTemplateError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	///
	pub async fn delete_jwt_template(clerk_client: &Clerk, template_id: &str) -> Result<crate::models::DeletedObject, Error<DeleteJwtTemplateError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/jwt_templates/{template_id}",
			local_var_configuration.base_path,
			template_id = crate::apis::urlencode(template_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<DeleteJwtTemplateError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve the details of a given JWT template
	pub async fn get_jwt_template(clerk_client: &Clerk, template_id: &str) -> Result<crate::models::JwtTemplate, Error<GetJwtTemplateError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/jwt_templates/{template_id}",
			local_var_configuration.base_path,
			template_id = crate::apis::urlencode(template_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetJwtTemplateError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	pub async fn list_jwt_templates(clerk_client: &Clerk) -> Result<Vec<crate::models::JwtTemplate>, Error<ListJwtTemplatesError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/jwt_templates", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<ListJwtTemplatesError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Updates an existing JWT template
	pub async fn update_jwt_template(
		clerk_client: &Clerk,
		template_id: &str,
		create_jwt_template_request: Option<crate::models::CreateJwtTemplateRequest>,
	) -> Result<crate::models::JwtTemplate, Error<UpdateJwtTemplateError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/jwt_templates/{template_id}",
			local_var_configuration.base_path,
			template_id = crate::apis::urlencode(template_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&create_jwt_template_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateJwtTemplateError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}
