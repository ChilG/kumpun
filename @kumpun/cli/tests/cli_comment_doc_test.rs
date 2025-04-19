mod common;
mod setup;

#[test]
fn test_struct_field_doc_comments_are_generated() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("everything.example");
    setup::run_generate(schemas, &["--with-docs"]);

    let content = common::read("tests/generated/everything_example.rs");

    // ตรวจ comment ของ field ทั่วไป
    assert!(content.contains(
        r#"
    /// Unique identifier (UUID) for the entity.
    /// Example: "550e8400-e29b-41d4-a716-446655440000"
    pub id: String,
    "#
    ));
    assert!(content.contains(
        r#"
    /// Indicates whether the user is currently active.
    /// Example: true
    pub is_active: Option<bool>,
    "#
    ));
    assert!(content.contains(
        r#"
    /// Mailing address of the user.
    /// Example: {"city":"Bangkok","zip":"10110"}
    pub address: Option<Address>,
    "#
    ));

    // ตรวจ comment ของ nested object
    assert!(content.contains(r#"
    /// Basic profile information.
    /// Example: {"address":{"line1":"123 Main Street","line2":"Apt 4B","zip":"10110"},"firstName":"Nantapon","lastName":"Sutha"}
    pub profile: Profile,
    "#));
    assert!(content.contains(
        r#"
/// Basic profile information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Detailed nested address for profile.
    /// Example: {"line1":"123 Main Street","line2":"Apt 4B","zip":"10110"}
    pub address: Option<ProfileAddress>,
    /// User's first name.
    /// Example: "Nantapon"
    pub first_name: String,
    /// User's last name.
    /// Example: "Sutha"
    pub last_name: String,
}"#
    ));

    // ตรวจ comment จาก enum field
    assert!(content.contains(
        r#"
    /// User status indicator.
    /// Example: "active"
    pub status: Status,
    "#
    ));
    assert!(content.contains(
        r#"

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Inactive,
    Banned,
}"#
    ));

    // ตรวจ comment จาก additionalProperties
    assert!(content.contains(
        r#"
    /// Arbitrary metadata as key-value pairs.
    /// Example: {"language":"th","nickname":"nanta"}
    pub meta: Option<HashMap<String, String>>,
    "#
    ));

    // ตรวจ comment ที่มาจาก oneOf
    assert!(content.contains(
        r#"
    /// User communication preferences.
    /// Example: {"email":"user@example.com"}
    pub preferences: Option<Preferences>,
    "#
    ));
    assert!(content.contains(
        r#"

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Preferences {
    /// User prefers email communication only.
    /// Example: {"email":"user@example.com"}
    EmailOnly(PreferencesEmailOnly),
    /// User prefers SMS communication only.
    /// Example: {"phone":"+66812345678"}
    SMSOnly(PreferencesSMSOnly),
}"#
    ));

    // ตรวจ comment ที่มาจาก allOf
    assert!(content.contains(
        r#"
    /// User configurable settings.
    /// Example: {"notifications":true,"theme":"dark"}
    pub settings: Option<Settings>,
    "#
    ));
    assert!(content.contains(
        r#"

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    /// UI theme setting.
    /// Example: {"theme":"dark"}
    #[serde(flatten)]
    pub part_1: SettingsPart1,
    /// Notification settings.
    /// Example: {"notifications":true}
    #[serde(flatten)]
    pub part_2: SettingsPart2,
}"#
    ));

    // ตรวจ comment ที่มาจาก anyOf
    assert!(content.contains(
        r#"
    /// Flexible identifier which could be a string or number.
    /// Example: "user_001"
    pub identifier: Option<Identifier>,
    "#
    ));
    assert!(content.contains(
        r#"

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Identifier {
    /// String-based identifier.
    /// Example: "user_001"
    Variant1(String),
    /// Integer-based identifier.
    /// Example: 1001
    Variant2(i32),
}"#
    ));
}
