#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EnctrytedCredentials,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub element_type: EncryptedPassportElementType,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub trasnlation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnctrytedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u64,
    pub file_date: u64,
}
