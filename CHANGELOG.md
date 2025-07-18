# Changelog

## [0.1.4](https://github.com/kohbis/spotter/compare/v0.1.3...v0.1.4) (2025-07-18)


### Features

* **cli:** add region validation and error handling for invalid AWS regions ([d062412](https://github.com/kohbis/spotter/commit/d062412593f6c1fe1e7301a0ef9f80379c9bc914))
* integrate anyhow for error handling and add anyhow dependency ([a52ae55](https://github.com/kohbis/spotter/commit/a52ae55b20b17ce8a09491f104ab30f3c2495aee))


### Build System

* **deps:** bump clap from 4.5.40 to 4.5.41 ([ddf90ab](https://github.com/kohbis/spotter/commit/ddf90ab1a55192c844630bb0a8aa4945b80899cc))
* **deps:** bump reqwest from 0.12.20 to 0.12.22 ([325fada](https://github.com/kohbis/spotter/commit/325fadafbd3a70f8e83395fb214d95cb208dc9f7))
* **deps:** bump tokio from 1.45.1 to 1.46.1 ([43cde69](https://github.com/kohbis/spotter/commit/43cde69bd6e6d9cca8926722152192facf64fdbe))

## [0.1.3](https://github.com/kohbis/spotter/compare/v0.1.2...v0.1.3) (2025-06-15)


### Features

* 🎸 implement AWS data fetching and display functionality ([b95bde8](https://github.com/kohbis/spotter/commit/b95bde819217b8dc57b21d45f4455d9b7e6740aa))


### Tests

* 🧪 add unit tests for CLI and display functionality ([3d033bb](https://github.com/kohbis/spotter/commit/3d033bba43e2274edd764a4b70851886056593fd))


### Build System

* **deps:** bump clap from 4.5.39 to 4.5.40 ([0e52708](https://github.com/kohbis/spotter/commit/0e5270805e208d9cee20823d047d81142d6bd301))
* **deps:** bump reqwest from 0.12.19 to 0.12.20 ([dcd4563](https://github.com/kohbis/spotter/commit/dcd456323d6311e22d40b16f77ef7343efde36ab))

## [0.1.2](https://github.com/kohbis/spotter/compare/v0.1.1...v0.1.2) (2025-06-01)


### Documentation

* Add Homebrew installation instructions to README ([4bc5c22](https://github.com/kohbis/spotter/commit/4bc5c2211cb6d4d4896bc3fa6c75410aec9894d8))


### Code Refactoring

* 💡 Modularize cli ([1b9250b](https://github.com/kohbis/spotter/commit/1b9250bc12871d7af550dd11d7269e0bac74dee9))

## [0.1.1](https://github.com/kohbis/spotter/compare/v0.1.0...v0.1.1) (2025-05-31)


### Features

* 🎸 Add LICENSE file and update package metadata in Cargo.toml ([749638b](https://github.com/kohbis/spotter/commit/749638b2aad0d585ec82484053ae756660c23665))
* 🎸 Add README file with project overview, features, installation, usage, and licensing information ([1704318](https://github.com/kohbis/spotter/commit/170431886d4b8fb6abe3bb446a8e38dceaa4569b))
* 🎸 Refactor logging and update dependencies in Cargo.toml and Cargo.lock ([e0694c0](https://github.com/kohbis/spotter/commit/e0694c00b5ad8f5417e943e49ca39b7a0a3004de))
* 🎸Enhance spot data processing by adding instance specifications for memory and cores ([50dfa01](https://github.com/kohbis/spotter/commit/50dfa0154729f6a1ccf76c16353329abd224e373))


### Continuous Integration

* 🎡 Add GitHub workflows for Dependabot, PR checks, and release automation ([1cbad95](https://github.com/kohbis/spotter/commit/1cbad95414c23a5fc65b4bc2fb96a9d01f7e279e))
