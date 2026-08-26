uniffi::setup_scaffolding!();
use thiserror::Error;

pub mod rasp_basic;

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

#[derive(uniffi::Object)]
pub struct MobileArmor {
    is_enterprise: bool,
}

#[uniffi::export]
impl MobileArmor {
    #[uniffi::constructor]
    pub fn initialize() -> Result<std::sync::Arc<Self>, ArmorError> {
        Ok(std::sync::Arc::new(Self { 
            is_enterprise: false, 
        }))
    }

    pub fn start_shield(&self) -> Result<(), ArmorError> {
        if rasp_basic::is_device_compromised() || rasp_basic::is_emulator() {
            return Err(ArmorError::DeviceCompromised);
        }
        if rasp_basic::is_debugger_attached() {
            return Err(ArmorError::DebuggerDetected);
        }
        Ok(())
    }
}
