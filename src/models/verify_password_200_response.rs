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
pub struct VerifyPassword200Response {
	#[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
	pub verified: Option<bool>,
}

impl VerifyPassword200Response {
	pub fn new() -> VerifyPassword200Response {
		VerifyPassword200Response { verified: None }
	}
}
