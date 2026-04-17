# To do

- [ ] All other command line flags in Cargo.toml
- [ ] Cache spdx detection to avoid re-running?
- [ ] Workspace support
    - [ ] Warn if no license files in workspace crates? what if not published?
- [ ] Add suggested fixes in the reports?
- [ ] more commands
    - [ ] Add a "add" command for quickly adding allow/warn/deny rules?
    - [ ] "prune" command for removing extra unneeded license
        - [ ] priority list for desired licenses? MIT > Apache?
- [ ] extra lints
    - [ ] Lint for unused allow/warn/deny flags/config
    - [ ] lint for deciding minimal set of licenses needed and warning on extra ones that aint needed
    - [ ] use root projects license expression to decide if copyleft is a problem?
    - [ ] Compare projects SPDX license (if any) with compatibility with licenses
    - [ ] Warn if no "license-file" file filled out in cargo

# Done
- [x] Don't use "tests" license to fix Pixar detection bug in identity: use a license file just for that in src
- [x] Lint name in report, so it can be more easily added to config file / command line by name
- [x] Consistent report order (by level, then alphabetically?)
- [x] Allow/warn/deny lints in Cargo.toml
- [x] Warn if no "license" file filled out in cargo

# Dont Do
- [ ] Combine "missing" and "unmet-spdx" alerts together?
- [ ] Don't list the number if only one of them?
