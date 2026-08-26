#!/bin/bash
set -e
echo "🚀 MobileArmor Android JNI Derlemesi Başlıyor..."
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo build --release --target aarch64-linux-android --features enterprise
cargo build --release --target armv7-linux-androideabi --features enterprise
cargo build --release --target x86_64-linux-android --features enterprise
mkdir -p out_android/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64}
cp target/aarch64-linux-android/release/libmobilearmor_core.so out_android/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libmobilearmor_core.so out_android/src/main/jniLibs/armeabi-v7a/
cp target/x86_64-linux-android/release/libmobilearmor_core.so out_android/src/main/jniLibs/x86_64/
echo "✅ BAŞARILI!"
