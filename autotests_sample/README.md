## configure launch threads
```shell
cargo test -- --test-threads=1
```

## configure test stdout
```shell
cargo test -- --show-output
```

## run test by name
```shell
cargo test custom_assert_test
cargo test this
```

## run ignored tests
```shell
cargo test -- --ignored
cargo test -- --include-ignored
```

## run integration tests
```shell
cargo test --test integration_tests
```