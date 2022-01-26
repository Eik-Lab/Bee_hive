<div align=center>
<img src="https://notion-emojis.s3-us-west-2.amazonaws.com/v0/svg-twitter/1f41d.svg" width=150>    
</div>

# Bee Hive, the IoT Bee cube backend

This API can be hosted in a service provider of your choice. We used heroku([guide](https://github.com/emk/heroku-buildpack-rust)), but you can use whatever you want.

To run the code, you need to have the following:

- [Rust](https://www.rust-lang.org/) installed on your system.  
   We recommend using [rustup](https://rustup.rs/)
   
- Postgresql
- An .env variable with the PORT and the DATABASE_URL to your database. E.g:

```
  PORT=3000
  DATABASE_URL=postgresql:///user:password@hostname:port/db
```

To run the code(for example on a local machine), run the following command in the same folder as the the github repo(might take some time to compile): `cargo run`

This will run the server on the machine, accessable by the ip of the machine, and the PORT than you declared earlier.

## Project members:

- Uzair Aftab (software developer)
- Patrick Pascal Bickenbach (hardware developer)
- Aleksander Eriksen (student project leader)

# TODO:

- Move migration instructions to this repository from Bee-CTRL
