# How to release `jsonrpsee`

1. Bump the version of all crates
1. Run all tests
1. In the `CHANGELOG.md` file, move everything under "Unreleased" to a new section named `## [vx.y.z] – YYYY-MM-DD`
1. Make a dryrun like so:
	1. Ensure you're in the project root dir
	1. Run: `./scripts/publish.sh --dry-run --allow-dirty`
