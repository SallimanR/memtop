test:
    cargo test -- --nocapture

test-2:
    cargo test -- --show-output

build-profile:
    cargo build --profile release-profile --features profile-with-tracy
