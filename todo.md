# To do

- [ ] Configuration in Cargo.toml
    - [x] Allow/warn/deny lints
    - [ ] All other command line flags
- [x] Consistent report order (by level, then alphabetically?)
- [x] Lint name in report, so it can be more easily added to config file / command line by name
- [ ] Cache spdx detection to avoid re-running?
- [x] Don't use "tests" license to fix Pixar detection bug in identity: use a license file just for that in src
- [ ] Workspace support