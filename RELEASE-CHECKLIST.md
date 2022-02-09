# How to release `jsonrpsee`

1. Bump the version of all crates
1. Run all tests
1. In the `CHANGELOG.md` file, move everything under "Unreleased" to a new section named `## [vx.y.z] – YYYY-MM-DD`
1. Publish: `./scripts/publish.sh`
1. Once published, make sure to "create a release" for the pushed tag on github.
