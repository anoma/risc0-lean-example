build:
    cd guest && lake build
    mkdir -p guest/.lake/packages/dummy/.lake/build/ir/
    mkdir -p guest_build/risc0_ir
    rsync -a --prune-empty-dirs --include '*/' --include '*.c' --exclude '*' guest/.lake/build/ir/ guest/.lake/packages/*/.lake/build/ir/ guest_build/risc0_ir/
    cd guest_build && just build
    cp guest_build/_build/libGuest.a methods/guest/lib/libGuest.a
    cargo build --release

clean:
    cd guest && lake clean
    cd guest_build && just clean
    rm -rf guest_build/risc0_ir/
    rm -f methods/guest/lib/libGuest.a
    cargo clean
