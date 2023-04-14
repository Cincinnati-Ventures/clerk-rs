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
pub struct UpdateUserMetadataRequest {
	/// Metadata saved on the user, that is visible to both your frontend and backend. The new object will be merged with the existing value.
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	/// Metadata saved on the user that is only visible to your backend. The new object will be merged with the existing value.
	#[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
	pub private_metadata: Option<serde_json::Value>,
	/// Metadata saved on the user, that can be updated from both the Frontend and Backend APIs. The new object will be merged with the existing value.  Note: Since this data can be modified from the frontend, it is not guaranteed to be safe.
	#[serde(rename = "unsafe_metadata", skip_serializing_if = "Option::is_none")]
	pub unsafe_metadata: Option<serde_json::Value>,
}

impl UpdateUserMetadataRequest {
	pub fn new() -> UpdateUserMetadataRequest {
		UpdateUserMetadataRequest {
			public_metadata: None,
			private_metadata: None,
			unsafe_metadata: None,
		}
	}
}
