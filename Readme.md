
## Compile for go

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

# Test the async bad function

```
cargo build --release --target x86_64-unknown-linux-musl
```

run testgo_async.go
```
go run testmuslasync/testgo_async.go
```


# Build for musl that fails:

```
cargo build --release --target x86_64-unknown-linux-musl
```

run testgo_musl.go
```
go run testmusl/testgo_musl.go
```

# Build for linux-gnu that works:

Tested on ubuntu 20.04 only, with x86_64-unknown-linux-gnu as default.
```
cargo build
```

run testgo_notmusl.go
```
go run testnotmusl/testgo_notmusl.go
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



