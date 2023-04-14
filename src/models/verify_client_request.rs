/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyClientRequest {
	/// A JWT Token that represents the active client.
	#[serde(rename = "token", skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
}

impl VerifyClientRequest {
	pub fn new() -> VerifyClientRequest {
		VerifyClientRequest { token: None }
	}
}
