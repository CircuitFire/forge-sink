# forge-sink
Easily let users match the mod list of a minecraft forge server.

## Client
- Place the client executable in the .minecraft folder.
- Create a shortcut to the client.
- Right click the shortcut and add the address to the end of the taget field. it should look something like this "C:\Users\USER\AppData\Roaming\.minecraft\sink-client.exe 192.168.1.99:25566"
- When you want to match the mods of the server just run the shortcut.
- If you have mods in you mod folder that the server does not it will warn you so can exit before it deletes them.
- If you are not running the same version of forge as the server it will drop the installer .jar file on you desktop so you can install it.

## Server
- Place the server executable in the same folder that the minecraft server is running from.
- Create a folder called "forge" and place the forge installer .jar inside it.
- run the server currently it runs on port 25566.

# build
- install Rust https://www.rust-lang.org
- open project directory in a terminal(The one that contains Cargo.toml)
- build and run
  - "cargo run"
- build
  - "cargo build" output binary = "./target/debug/sink-client", "./target/release/sink-server"
  - or "cargo build --release" output binary = "./target/release/sink-client", "./target/release/sink-server"
