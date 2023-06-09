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
pub struct UpdateUserRequest {
	/// The ID of the user as used in your external systems or your previous authentication solution. Must be unique across your instance.
	#[serde(
		rename = "external_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub external_id: Option<Option<String>>,
	/// The first name to assign to the user
	#[serde(
		rename = "first_name",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub first_name: Option<Option<String>>,
	/// The last name to assign to the user
	#[serde(
		rename = "last_name",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub last_name: Option<Option<String>>,
	/// The ID of the email address to set as primary. It must be verified, and present on the current user.
	#[serde(rename = "primary_email_address_id", skip_serializing_if = "Option::is_none")]
	pub primary_email_address_id: Option<String>,
	/// The ID of the phone number to set as primary. It must be verified, and present on the current user.
	#[serde(rename = "primary_phone_number_id", skip_serializing_if = "Option::is_none")]
	pub primary_phone_number_id: Option<String>,
	/// The ID of the web3 wallets to set as primary. It must be verified, and present on the current user.
	#[serde(rename = "primary_web3_wallet_id", skip_serializing_if = "Option::is_none")]
	pub primary_web3_wallet_id: Option<String>,
	/// The username to give to the user. It must be unique across your instance.
	#[serde(
		rename = "username",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub username: Option<Option<String>>,
	/// The ID of the image to set as the user's profile image
	#[serde(
		rename = "profile_image_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub profile_image_id: Option<Option<String>>,
	/// The plaintext password to give the user. Must be at least 8 characters long, and can not be in any list of hacked passwords.
	#[serde(
		rename = "password",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub password: Option<Option<String>>,
	/// In case TOTP is configured on the instance, you can provide the secret to enable it on the specific user without the need to reset it. Please note that currently the supported options are: * Period: 30 seconds * Code length: 6 digits * Algorithm: SHA1
	#[serde(rename = "totp_secret", skip_serializing_if = "Option::is_none")]
	pub totp_secret: Option<String>,
	/// If Backup Codes are configured on the instance, you can provide them to enable it on the specific user without the need to reset them. You must provide the backup codes in plain format or the corresponding bcrypt digest.
	#[serde(rename = "backup_codes", skip_serializing_if = "Option::is_none")]
	pub backup_codes: Option<Vec<String>>,
	/// Metadata saved on the user, that is visible to both your Frontend and Backend APIs
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	/// Metadata saved on the user, that is only visible to your Backend API
	#[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
	pub private_metadata: Option<serde_json::Value>,
	/// Metadata saved on the user, that can be updated from both the Frontend and Backend APIs. Note: Since this data can be modified from the frontend, it is not guaranteed to be safe.
	#[serde(rename = "unsafe_metadata", skip_serializing_if = "Option::is_none")]
	pub unsafe_metadata: Option<serde_json::Value>,
	/// A custom date/time denoting _when_ the user signed up to the application, specified in RFC3339 format (e.g. `2012-10-20T07:15:20.902Z`).
	#[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
	pub created_at: Option<String>,
}

impl UpdateUserRequest {
	pub fn new() -> UpdateUserRequest {
		UpdateUserRequest {
			external_id: None,
			first_name: None,
			last_name: None,
			primary_email_address_id: None,
			primary_phone_number_id: None,
			primary_web3_wallet_id: None,
			username: None,
			profile_image_id: None,
			password: None,
			totp_secret: None,
			backup_codes: None,
			public_metadata: None,
			private_metadata: None,
			unsafe_metadata: None,
			created_at: None,
		}
	}
}
