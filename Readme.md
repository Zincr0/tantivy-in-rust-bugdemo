
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

build

```
cargo build --release --target x86_64-unknown-linux-musl
```

run testgo.go
```
go run testgo.go
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



