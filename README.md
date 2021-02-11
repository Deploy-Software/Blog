# Deploy Software Blogs

Deploy Software Blogs is a blogging platform written in Rust, including both the frontend and the backend.

## Getting Started


These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites


What things you need to install the software and how to install them

* PostgreSQL

### Installing

Export a Database URL. For example:

`export DATABASE_URL="postgres://john:doe@localhost/blog"`

`./start.sh`

Then browse to http://\<yourhost\>:\<yourport\>

Typically http://localhost:3030.

### Running the tests
Simply run

`cd app``
cargo test

cd ../server
cargo test`````

### And coding style tests

The whole project is styled according to the Rust Formatter.

## Deployment
```
cargo run --release
```

## Built With

* [Warp](https://github.com/seanmonstar/warp) - The Backend Framework.
* [Yew](https://yew.rs/) - The Frontend Framework.

## Authors

* **Axel Nilsson** - [GitHub](https://github.com/AxelNilsson)

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE.md](LICENSE.md) file for details

