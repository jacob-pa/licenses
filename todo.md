# To do

- [ ] All other command line flags in Cargo.toml
- [ ] Cache spdx detection to avoid re-running?
- [ ] Workspace support
    - [ ] Warn if no license files in workspace crates? what if not published?
- [ ] Add suggested fixes in the reports?
- [ ] Add a "add" command for quickly adding allow/warn/deny rules?
- [ ] extra lints
    - [ ] Lint for unused allow/warn/deny flags/config
    - [ ] lint for deciding minimal set of licenses needed and warning on extra ones that aint needed
    - [ ] use root projects license expression to decide if copyleft is a problem?
    - [ ] Compare projects SPDX license (if any) with compatibility with licenses
    - [ ] Warn if no "license-file" file filled out in cargo
- [ ] Error for "cargo run -- prune INVALID" is weirdly formatted
- [ ] Make sure remote licenses from repos are from the right commit / version
    - [ ] Get some kind of version for github licenses?
- [ ] Handle license exceptions properly (what even are they?)
- [ ] Only consider license "of package" if same name AND same version
- [ ] "Excluded" ids in 

# Done
- [x] Don't use "tests" license to fix Pixar detection bug in identity: use a license file just for that in src
- [x] Lint name in report, so it can be more easily added to config file / command line by name
- [x] Consistent report order (by level, then alphabetically?)
- [x] Allow/warn/deny lints in Cargo.toml
- [x] Warn if no "license" file filled out in cargo
- [x] "prune" command for removing extra unneeded license
    - [x] priority list for desired licenses e.g. MIT > Apache
- [x] Version numbers in stored license name
- [x] Conservative prune: only licenses that are in the SPDX expression but are not needed, leave unknown types.
- [x] Put version numbers in file name! Dependencies may have different licenses at different versions!

# Dont Do
- [ ] Combine "missing" and "unmet-spdx" alerts together?
- [ ] Don't list the number if only one of them?
