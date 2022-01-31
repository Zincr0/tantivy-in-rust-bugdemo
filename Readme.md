
## Compile for go, alternative 1

Use Cargo.toml with
```
[lib]
name = "failgosample"
crate_type = ["staticlib"]
```

install x86_64-unknown-linux-musl
```
rustup target add x86_64-unknown-linux-musl
```

install musl
```
sudo apt-get install musl musl-tools musl-dev
```

build

```
cargo build --release --target x86_64-unknown-linux-musl
```

# Compile for go, alternative 2

Use Cargo.toml with
```
[lib]
name = "failgosample"
crate_type = ["staticlib"]
```

Use docker
```
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release
```

# Test the async function, alternative 1

Run go using musl
```
CC=/usr/bin/musl-gcc go run --ldflags '-linkmode external -extldflags "-static"' testmuslasync/testgo_async.go
```


# Test the async function, alternative 2

Use docker
```
docker run --rm -v "$PWD":/usr/src/myapp -w /usr/src/myapp tetafro/golang-gcc:1.17-alpine go run testmuslasync/testgo_async.go
```

## Compile for python

Use Cargo.toml with
```
[lib]
name = "failgosample"
crate-type = ["cdylib"] 
```

build

```
cargo build --release
```

run testpython.py
```
python3 testpython.py
```



