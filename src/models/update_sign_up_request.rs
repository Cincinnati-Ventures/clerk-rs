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
pub struct UpdateSignUpRequest {
	/// Specifies whether a custom action has run for this sign-up attempt. This is important when your instance has been configured to require a custom action to run before converting a sign-up into a user. After executing any external business logic you deem necessary, you can mark the sign-up as ready-to-convert by setting `custom_action` to `true`.
	#[serde(rename = "custom_action", skip_serializing_if = "Option::is_none")]
	pub custom_action: Option<bool>,
	/// The ID of the guest attempting to sign up as used in your external systems or your previous authentication solution. This will be copied to the resulting user when the sign-up is completed.
	#[serde(
		rename = "external_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub external_id: Option<Option<String>>,
}

impl UpdateSignUpRequest {
	pub fn new() -> UpdateSignUpRequest {
		UpdateSignUpRequest {
			custom_action: None,
			external_id: None,
		}
	}
}
