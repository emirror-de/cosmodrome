Authentication and authorization library for [rocket](https://rocket.rs).

## Introduction and purpose

`cosmodrome` provides a customizable and extendable way to secure your rocket application from unauthorized access to your resources/routes.
This crate contains the required data structures for usage on server and client side.

By default, both features `server` and `client` are enabled which are required to use this library on the server side.

## Client usage

For the use on the client side (eg. WASM), use `default-features = false` and `features = ["client"]`. This will only include the `Ticket` model.

## How does it work?

`cosmodrome` in the broadest sense follows the idea of a boarding and traveling process using an airplane.
To get access to the airplane, you need to pass the [gate](gate::Gate). This [gate](gate::Gate) is able to log you in and out from our application.
If you want to buy a [Ticket] it is required that you have a valid [passport](passport::Passport). This [passport](passport::Passport)
contains details about yourself and is usually referred to as "account". Your application requires a [passport register](passport_register::PassportRegister)
that holds all [passport](passport::Passport)s/accounts.
To successfully pass the [gate](gate::Gate) you need to have a valid [passport](passport::Passport) as well as [Ticket] that you bought earlier.
In our case, the [Ticket] is a combination of an `id` and a `secret`, where `id` matches [the passport's id](passport::Passport::id).
Now the [gate](gate::Gate) can verify wether your [Ticket] is correct and, on success, provide you with a [boarding pass](boarding_pass::BoardingPass) that enables
access to all airplanes (aka webservice routes) you require to reach your final destination airport.

### Additional information

To be able to verify that you do have a correct [boarding pass](boarding_pass::BoardingPass) while traveling, it is required that you store it in
a [storage](storage::Storage) which is usually your hand luggage. `cosmodrome` brings build in support for [bearer token](auth_type::Bearer) as well as [cookie](auth_type::Cookie). Both
of them do have a [JWT payload](boarding_pass::payloads::JsonWebToken) that contains your [passport](passport::Passport). It is also possible that you implement your own [auth_type] and a custom payload by
implementing [BoardingPassStorage](storage::BoardingPassStorage).

## Features

The following methods are currently provided:

* Bearer
* Cookie

Both methods are using `JWT` as payload.

## Examples

Examples are provided in the `examples` folder in the repository.

## Attributions

Many thanks to the creator of the logo image:

Cosmodrome icons created by Kalashnyk - Flaticon - [https://www.flaticon.com/free-icons/cosmodrome](https://www.flaticon.com/free-icons/cosmodrome)
