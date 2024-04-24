# RIMS-Code website scheme submission

This is the source code repository for the RIMS-Code website scheme submission application.
The application is written in Rust and uses the 
[eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

## Maintainer e-mails

The maintainer e-mails for the `Submit via e-mail` function are stored
in `lib.rs` in the `DB_MAINTAINER_EMAIL` constant.
To add more than one e-mail,
separate them with a semicolon `;`.

## Deployments

Upon pushing to the `main` branch,
the new version of the app is automatically deployed to the
`gh-pages` branch of this repository.
Furthermore, that page is bound into the `rims-code.github.io` website
via an `iframe`, thus,
after the build and deployment process is finished,
the new version of the app is available automatically.

## Development

The project is based on the [egui/eframe template](https://github.com/emilk/eframe_template).
See the `README.md` in the `eframe_template` directory for more information
on building and local deployments.
