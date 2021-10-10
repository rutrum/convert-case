test:
    cargo test --all

watch-test:
    watchexec -- "reset && just test"

build:
    cargo build --all

watch-build:
    watchexec -- "reset && just build"

coverage:
    cargo tarpaulin --all-features --out Xml && pycobertura show cobertura.xml

doc:
    cargo doc

watch-doc:
    watchexec -- just doc
