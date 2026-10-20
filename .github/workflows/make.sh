#!/usr/bin/env bash

function _setup
{
    if [[ -f '/etc/os-release' ]]; then
        source '/etc/os-release'
        if ! pkg-config --exists elementary; then
            declare -ra DEPS=(sh{fmt,ellcheck} pkg-config)
            case ${ID:?} in
                msys2) return 0 ;;
                fedora | alma) sudo dnf install -y "${DEPS[@]}" {efl,clang}-devel ;;
                debian | ubuntu)
                    sudo apt-get update
                    sudo apt-get install -y "${DEPS[@]}" lib{efl-all,clang}-dev clang xvfb
                    ;;
            esac 1>/dev/null
            shellcheck --external-sources "${0}"
            shfmt -ci -fn -i 4 -d "${0}"
        fi
        if [[ ${ID} == debian || ${ID} == ubuntu ]] && ! command -v xvfb-run >/dev/null 2>&1; then
            sudo apt-get update
            sudo apt-get install -y xvfb
        fi
    fi
}

set -xeuo pipefail
if ((${#})); then
    case ${1} in
        setup) _setup ;;
        build)
            cargo check --workspace --no-default-features
            cargo check --workspace --all-features
            cargo clippy --quiet --workspace --lib --examples
            cargo test --workspace --lib
            cargo doc --workspace --all-features --no-deps
            cargo build --release --workspace --examples
            cargo fmt --check --all
            ;;
    esac
fi
