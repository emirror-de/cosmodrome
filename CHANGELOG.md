# Changelog
## v0.2.0
Release date: *2024-08-23*

### Global changes

#### ☯ Version specifics

- Updated Cargo files to 0.2.0 [be38483]

#### 🏗  Refactor

- Added account, Cookie name and JWT encryption can now be set using AuthSettings [4862d87]
- Implemented initial working version with simple example [4452857]
- Renamed everything for the new name rocket-airport [41fba33]
- Renamed AirportSettings to AirportConfig [5c175a8]
- Renamed crate to cosmodrome, Renamed immigration to gate [cefb42f]
- Added features server and client, server is default [039d1bb]
- Made the API more flexible [a401c70]
- Moved modules around for better overview [41a448c]
- Moved all modules enabled by server feature to server module [18bbfb5]

#### 🐞 Bug Fixes

- Increased `exp` value of JWT from one minute to one week [2562052]
- Removed &self parameter from Gate::logout [c5ac1f9]
- Renamed duplicated example name to bearer [f10169f]
- Added additional libiconv for aarch64-darwin [26b1149]

#### 📄 Documentation

- Updated documentation, Added deny missing docs to crate [cab2c1d]
- Updated README.md [e32f37d]
- Fixed unresolved link to AUTHORIZATION [85a733f]
- Updated documentation of auth_type module [9332f5a]
- Update Storage documentation [572fa59]
- Added icon [b43595c]
- Added attribution to logo creator [41843fc]
- Updated introduction and purpose section in README.md [c78d307]
- Updated README.md [4166204]

#### 🚲 Miscellaneous Tasks

- Renamed user_claims to account claims [9bfa317]
- Added old account model to enable development [107fd0d]
- Added required values to Cargo.toml [151c9ea]
- Removed ununsed undraw illustration [d11a4a9]
- Added Cargo.lock [324e1ba]
- Default expires_at for a passport is now 104 weeks [e2e10d0]
- Ticket model is now behind the client feature [1c073d5]
- Ticket is now using serde instead of rocket::serde [1921294]
- Renamed tab in development environment [17df0fd]
- Updated layout.kdl [66fdee2]
- Added examples [bef6804]
- Applied clippy fix [cdaa117]
- Further cleanup, updated examples [32d2d18]
- Removed some unnecessary codes and comments [43f01ed]
- Removed unncessary logout implementation for cookie and bearer auth type [cb83374]
- Updated Cargo.lock [eacc8ee]
- Added rust-toolchain.toml [a416635]
- Added .DS_Store to .gitignore [a3d5c6f]
- Now requiring nightly toolchain because of doc_cfg [4fbb21b]
- Removed client feature requirement for Ticket [0c235d7]
- Removed nushell from flake as well as RUSTUP_TOOLCHAIN env in shellHook [01a5c98]

#### 🛳  Features

- Added cliff and rustfmt toml files [c827d66]
- Added new function to Ticket [be582f7]
- Added support for cookie and bearer auth [636c1ff]
- Added Gate::logout method [8f8b5f9]
- Added default expiration time of one week to CookieStorageOptions [f92b44c]


