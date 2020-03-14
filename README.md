```
cargo build
echo "" | ./target/debug/rust_8cc > tmp.s
docker run -w /home -v ${PWD}:/home --rm -it alpine_c_lang gcc -o tmp.out driver.c tmp.s
docker run -w /home -v ${PWD}:/home --rm -it alpine_c_lang ./tmp.out
rm -f tmp.out tmp.s
```
