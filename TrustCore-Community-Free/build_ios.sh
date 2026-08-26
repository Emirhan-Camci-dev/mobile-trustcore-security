#!/bin/bash
set -e
echo "🚀 MobileArmor iOS XCFramework Derlemesi Başlıyor..."
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
cargo build --release --target aarch64-apple-ios --features enterprise
cargo build --release --target x86_64-apple-ios --features enterprise
cargo build --release --target aarch64-apple-ios-sim --features enterprise
mkdir -p out_ios
lipo -create target/x86_64-apple-ios/release/libmobilearmor_core.a target/aarch64-apple-ios-sim/release/libmobilearmor_core.a -output target/libmobilearmor_core_sim.a
cargo run --bin uniffi-bindgen generate core-community/src/mobilearmor.udl --language swift --out-dir out_ios || echo "UniFFI makro kullanildigi icin UDL gerekmiyor olabilir, lib.rs icinden generate edilecek."
rm -rf out_ios/MobileArmor.xcframework
xcodebuild -create-xcframework -library target/aarch64-apple-ios/release/libmobilearmor_core.a -headers out_ios -library target/libmobilearmor_core_sim.a -headers out_ios -output out_ios/MobileArmor.xcframework
echo "✅ BAŞARILI!"
