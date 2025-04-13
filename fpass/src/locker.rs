

pub struct PasswordLocker {
    pub salt: [u8; 8],
    pub key: [u8; 32],
    pub iv: [u8; 16],
}