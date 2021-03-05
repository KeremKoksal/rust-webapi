# WEB-API with Rust
This project was created for educational purposes using [actix](https://actix.rs) and [diesel](https://diesel.rs). 

## Useful Repos

https://github.com/rust-lang/rustlings

https://github.com/rust-unofficial/awesome-rust

https://github.com/nemesiscodex/user-auth

https://github.com/JasterV/actix-web-posts-api

https://github.com/saschagrunert/webapp.rs


## Usage

```
sudo apt-get update

sudo apt-get install -y libpq-dev

cargo install diesel_cli --no-default-features --features postgres

docker-compose up -d 

diesel migration run

sudo apt-get install gcc # required for cargo watch

cargo install cargo-watch 

cargo watch -x run

```


## Help
[Actix Gitter](https://gitter.im/actix/actix), [Actix Docs](https://actix.rs/docs/)

[Diesel Gitter](https://gitter.im/diesel-rs/diesel), [Diesel Guides](https://diesel.rs/guides/)



## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)