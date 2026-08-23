# Mobile TrustCore Security 🛡️

**Enterprise-Grade Zero-Trust Mobile App Protection SDK (RASP)**

![Rust](https://img.shields.io/badge/Made_with-Rust-black?logo=rust&style=for-the-badge)
![iOS](https://img.shields.io/badge/Platform-iOS-blue?logo=apple&style=for-the-badge)
![Android](https://img.shields.io/badge/Platform-Android-green?logo=android&style=for-the-badge)
![License](https://img.shields.io/badge/License-Dual_Licensed-red?style=for-the-badge)

Mobile TrustCore Security provides real-time **Runtime Application Self-Protection (RASP)**, **Memory Encryption**, and **Hardware-Attested Environment Integrity** for high-security iOS and Android applications. 

Designed strictly for FinTech, Neobanks, and Healthcare apps to ensure PCI-DSS, PSD2, and strict banking compliance with **sub-1ms runtime overhead**.

---

## ⚠️ Licensing & Commercial Use (Dual-License)

TrustCore follows a strict **Dual-Licensing (Open-Core)** model.

### 🟢 Community Edition (AGPLv3)
This repository contains the Community Edition. It is 100% free and open-source. However, due to the viral nature of the AGPLv3 license, **if you include this SDK in a closed-source proprietary app on the App Store or Play Store, you are legally required to open-source your entire application.**

### 🔴 Pro / Enterprise Edition (Commercial)
If you are building a commercial application and cannot share your source code, you **MUST** purchase a commercial license. The Pro edition includes military-grade evasion tactics, in-memory encryption, and hardware keystore verification.

👉 **[Purchase Enterprise License via Polar.sh](https://buy.polar.sh/polar_cl_ZVa5iTx304DqXrqHHYiSAOKmPGReXSrhR1cOB0cXIXJ)**

👉 **[Access the Pro Repository (License Required)](https://github.com/Emirhan-Camci-dev/mobile-trustcore-security-pro)**

---

## 🌟 Security Feature Matrix

| Defense Mechanism | Community (AGPLv3) | Pro / Enterprise |
| :--- | :---: | :---: |
| **Basic Jailbreak & Root Indicators** | ✅ | ✅ |
| **Standard Anti-Debugging (ptrace)** | ✅ | ✅ |
| **Compile-Time String Obfuscation** | ❌ | ✅ |
| **Advanced Frida & Xposed Scanning** | ❌ | ✅ |
| **Raw Syscalls (Bypass libc hooks)** | ❌ | ✅ |
| **Zygisk / KernelSU / Magisk Hide Detection**| ❌ | ✅ |
| **In-Memory Data Vault (ChaCha20-Poly1305)**| ❌ | ✅ |
| **Secure Enclave / StrongBox Auth**| ❌ | ✅ |
| **Offline Ed25519 License Verification** | ❌ | ✅ |

---

## ⚡ Performance Benchmarks (ARM64)

Written entirely in memory-safe Rust and C++20. TrustCore bypasses high-level abstractions, communicating directly with the kernel to ensure zero battery drain and unnoticeable latency.

| Execution Phase | Average Latency | Memory Overhead |
| :--- | :--- | :--- |
| SDK Initialization | `0.45 ms` | `< 2 MB` |
| RASP Integrity Loop | `0.12 ms` | `0 MB (Zero-allocation)`|
| In-Memory Encryption | `0.08 ms` | `Zero-copy` |

---

## 🚀 Quickstart (Pro Edition)

TrustCore is designed for a "Fail-Secure" experience. It doesn't crash your app silently; it triggers a secure callback so you can log the threat to your servers before shutting down.

### Swift (iOS)
```swift
import MobileArmor
import CryptoKit

// 1. Initialize with your cryptographic offline license
let armor = try MobileArmor.initialize(
    licenseKey: "YOUR_OFFLINE_JWT_FROM_POLAR", 
    bundleId: "com.bank.app",
    expectedSignatureHash: "A1:B2:C3:D4",
    keystore: AppleSecureEnclaveKeystore()
)

// 2. Start the shield and listen for threats
try armor.startShield { threat in 
    Analytics.logEvent("security_violation", parameters: ["type": threat.description])
    fatalError("Security Violation: \(threat.description)") 
}

// 3. Encrypt sensitive data in RAM (Data Vault)
let safeData = try armor.encryptInMemory(plaintext: "CREDIT_CARD_DATA".data(using: .utf8)!)
```

### Kotlin (Android)
```kotlin
import com.trustcore.mobilearmor.MobileArmor

// 1. Initialize the SDK
val armor = MobileArmor.init(
    context, 
    "YOUR_OFFLINE_JWT_FROM_POLAR", 
    "com.bank.app",
    "A1:B2:C3:D4",
    AndroidHardwareKeystore()
)

// 2. Start the shield
armor.startShield { threat -> 
    FirebaseCrashlytics.getInstance().log("Threat: ${threat.name}")
    throw SecurityException("Security Violation: ${threat.name}") 
}
```

---
**Author:** Emirhan CAMCI | **Email:** byemir@live.com | **Year:** 2026
