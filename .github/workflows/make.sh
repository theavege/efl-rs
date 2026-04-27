set -euo pipefail
source '/etc/os-release'
case ${ID:?} in
    debian | ubuntu) sudo bash -c '
        apt-get update
        apt-get install -y libefl-all-dev
    ';;
    fedora | alma) sudo dnf install -y efl-devel ;;
esac 1> /dev/null
cargo clippy --quiet --example simple
cargo build --release --example simple
cargo fmt --check --all
