STEPS TO BUILD AND RUN THE PROGRAM
```shell
rustc --crate-type=lib blog.rs
```

```shell
rustc main.rs --extern blog=libblog.rlib && ./main
```
