#!/usr/bin/env bash

function _setup
{
    if [[ -f '/etc/os-release' ]]; then
        source '/etc/os-release'
        if ! command -v efl-config >/dev/null && ! pkg-config --exists elementary; then
            declare -ra DEPS=(sh{fmt,ellcheck} pkg-config)
            case ${ID:?} in
                debian | ubuntu)
                    sudo apt-get update
                    sudo apt-get install -y "${DEPS[@]}" libefl-all-dev clang libclang-dev
                    ;;
                fedora | alma) sudo dnf install -y "${DEPS[@]}" efl-devel clang-devel ;;
            esac 1>/dev/null
            shellcheck --external-sources "${0}"
            shfmt -ci -fn -i 4 -d "${0}"
        fi
    fi
}

set -euo pipefail
if ((${#})); then
    case ${1} in
        setup) _setup ;;
        build)
            cargo clippy --quiet --workspace --lib --examples --features=all
            cargo test --workspace --lib
            cargo build --release --workspace --examples --features=all
            cargo fmt --check --all
            ;;
    esac
fi
