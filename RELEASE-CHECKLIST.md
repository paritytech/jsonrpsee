# How to release `jsonrpsee`

1. Bump the version of all crates
1. Run all tests
1. In the `CHANGELOG.md` file, move everything under "Unreleased" to a new section named `## [vx.y.z] – YYYY-MM-DD`
1. Make a dryrun like so:
	1. Ensure you're in the project root dir
	1. Run: `./scripts/publish.sh --dry-run`
	Note: the script will publish the crates in the correct order and pause after each crate to ensure it's available at the crates registry before proceeding. This means the dry run isn't as useful and will end up in an infinite loop. If you're really unsure about the changes and want to do a dry run you should do a `cargo publish --dry-run` for each individual crate.
1. Publish: `./scripts/publish.sh`
1. Once published, make sure to "create a release" for the pushed tag on github. 
