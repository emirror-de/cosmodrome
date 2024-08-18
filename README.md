Authentication and authorization library for [rocket](https://rocket.rs).

## Introduction and purpose

`cosmodrome` provides a customizable way to secure your rocket application from unauthorized access to your resources/routes.
This crate contains the required data structures for usage on server and client side.

By default, both features `server` and `client` are enabled which are required to use this library on the server side.

## Client usage

For the use on the client side (eg. WASM), use `default-features = false` and `features = ["client"]`. This will only include the `Ticket` model.

## Attributions

Many thanks to the creator of the logo image:

Cosmodrome icons created by Kalashnyk - Flaticon - [https://www.flaticon.com/free-icons/cosmodrome](https://www.flaticon.com/free-icons/cosmodrome)
