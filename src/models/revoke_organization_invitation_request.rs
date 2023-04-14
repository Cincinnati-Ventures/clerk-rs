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
pub struct RevokeOrganizationInvitationRequest {
	/// The ID of the user that revokes the invitation. Must be an administrator in the organization.
	#[serde(rename = "requesting_user_id")]
	pub requesting_user_id: String,
}

impl RevokeOrganizationInvitationRequest {
	pub fn new(requesting_user_id: String) -> RevokeOrganizationInvitationRequest {
		RevokeOrganizationInvitationRequest { requesting_user_id }
	}
}
