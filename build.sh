cross build -p outer --release --target=aarch64-linux-android

mkdir -p kotlin/app/src/main/jniLibs/arm64-v8a
cp ./target/aarch64-linux-android/release/libouter.so ./kotlin/app/src/main/jniLibs/arm64-v8a/libouter.so

cargo run -p uniffi-bindgen generate \
  ./kotlin/app/src/main/jniLibs/arm64-v8a/libouter.so \
  --library \
  --language kotlin \
  --no-format \
  --out-dir kotlin/app/src/main/java

