set -euo pipefail
source '/etc/os-release'
case ${ID:?} in
    debian | ubuntu) sudo bash -c '
        apt-get update
        apt-get install -y libefl-all-dev
    ';;
    fedora | alma) sudo dnf install -y efl-devel ;;
esac 1> /dev/null
cargo clippy --quiet --features="all" --examples
cargo build --release --features="all" --examples
cargo fmt --check --all
