uniffi::setup_scaffolding!();
use thiserror::Error;

pub mod rasp_basic;

#[cfg(feature = "enterprise")]
use enterprise_ext::{license_validator, rasp_advanced, watchdog, memory_vault};

#[derive(Error, Debug, uniffi::Error)]
pub enum ArmorError {
    #[error("Lisans gecersiz, suresi dolmus veya manipule edilmis.")]
    LicenseInvalid,
    #[error("Cihaz guvenligi ihlali: Root, Jailbreak veya Emulator tespit edildi!")]
    DeviceCompromised,
    #[error("Guvenlik ihlali: Debugger baglantisi tespit edildi!")]
    DebuggerDetected,
    #[error("Guvenlik ihlali: Bellek manipulasyonu (Frida) tespit edildi!")]
    MemoryHookDetected,
    #[error("Guvenlik ihlali: Uygulama sahte bir sertifika ile yeniden imzalanmis (Repackaged)!")]
    RepackagedApp,
    #[error("Kriptografi hatasi: Veri sifrelenemedi veya cozulemedi.")]
    CryptoError,
    #[error("Donanim (Hardware Keystore) Hatasi")]
    HardwareKeystoreError,
}

// 4. DONANIM DESTEKLİ KRİPTOGRAFİ (SECURE ENCLAVE / STRONGBOX)
// UniFFI "with_foreign" makrosu ile bu arayüzü (Interface) dışarı açarız.
// Swift geliştiricisi bunu CryptoKit ile, Kotlin geliştiricisi AndroidKeyStore ile doldurur.
#[uniffi::export(with_foreign)]
pub trait HardwareKeystore: Send + Sync {
    /// Donanım çipi (Secure Enclave) içindeki Private Key ile veriyi imzalar.
    fn sign_payload(&self, payload: Vec<u8>) -> Result<Vec<u8>, ArmorError>;
    
    /// Donanım çipinden üretilen Public Key'i döndürür.
    fn get_public_key(&self) -> Result<Vec<u8>, ArmorError>;
}

#[derive(uniffi::Object)]
pub struct MobileArmor {
    is_enterprise: bool,
    expected_hash: String,
    session_key: Vec<u8>,
    hardware_keystore: Option<std::sync::Arc<dyn HardwareKeystore>>,
}

#[uniffi::export]
impl MobileArmor {
    #[uniffi::constructor]
    pub fn initialize(
        license_key: String, 
        bundle_id: String, 
        expected_signature_hash: String,
        keystore: Option<std::sync::Arc<dyn HardwareKeystore>>
    ) -> Result<std::sync::Arc<Self>, ArmorError> {
        #[cfg(feature = "enterprise")]
        {
            if !license_validator::verify_offline_license(&license_key, &bundle_id) {
                return Err(ArmorError::LicenseInvalid);
            }
            let key = memory_vault::generate_session_key();
            Ok(std::sync::Arc::new(Self { 
                is_enterprise: true, 
                expected_hash: expected_signature_hash, 
                session_key: key,
                hardware_keystore: keystore
            }))
        }
        #[cfg(not(feature = "enterprise"))]
        {
            if license_key.is_empty() { return Err(ArmorError::LicenseInvalid); }
            Ok(std::sync::Arc::new(Self { 
                is_enterprise: false, 
                expected_hash: expected_signature_hash, 
                session_key: vec![],
                hardware_keystore: keystore
            }))
        }
    }

    pub fn start_shield(&self) -> Result<(), ArmorError> {
        if rasp_basic::is_device_compromised() || rasp_basic::is_emulator() {
            return Err(ArmorError::DeviceCompromised);
        }
        if rasp_basic::is_debugger_attached() {
            return Err(ArmorError::DebuggerDetected);
        }

        #[cfg(feature = "enterprise")]
        {
            if !rasp_advanced::verify_app_signature(&self.expected_hash) { return Err(ArmorError::RepackagedApp); }
            if rasp_advanced::detect_advanced_root_hiding() { return Err(ArmorError::DeviceCompromised); }
            if rasp_advanced::scan_memory_maps() { return Err(ArmorError::MemoryHookDetected); }
            
            // Eğer donanımsal keystore tanımlanmışsa donanım bütünlüğünü sına
            if let Some(keystore) = &self.hardware_keystore {
                let test_payload = b"MobileArmor_Hardware_Test".to_vec();
                if keystore.sign_payload(test_payload).is_err() {
                    return Err(ArmorError::HardwareKeystoreError);
                }
            }

            watchdog::spawn_security_watchdog();
        }

        Ok(())
    }

    pub fn encrypt_in_memory(&self, plaintext: Vec<u8>) -> Result<Vec<u8>, ArmorError> {
        #[cfg(feature = "enterprise")]
        {
            memory_vault::encrypt_payload(&self.session_key, &plaintext)
                .map_err(|_| ArmorError::CryptoError)
        }
        #[cfg(not(feature = "enterprise"))]
        {
            Ok(plaintext)
        }
    }

    pub fn decrypt_in_memory(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, ArmorError> {
        #[cfg(feature = "enterprise")]
        {
            memory_vault::decrypt_payload(&self.session_key, &ciphertext)
                .map_err(|_| ArmorError::CryptoError)
        }
        #[cfg(not(feature = "enterprise"))]
        {
            Ok(ciphertext)
        }
    }
}
