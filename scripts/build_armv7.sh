use_nightly=false

# exit when any command fails
set -e

cd ..

sudo apt install gcc-arm-linux-gnueabi #reboot required
#sudo reboot

rust_version="stable"
if $use_nightly
then
  rust_version="nightly"
fi

rustup install $rust_version
rustup default $rust_version

rustup target add armv7-unknown-linux-gnueabi

cargo +$rust_version rustc --package mouse-keyboard-input --example mouse --target armv7-unknown-linux-gnueabi --release
