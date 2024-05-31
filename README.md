# Convert markdown files to html Rust exercise

In my journey of learning Rust, I decided to pick a small Python program that converts markdown files to html + makes an index page for those files, and rewrite it in Rust.

To learn the syntax and also see if I could speed it up.

Seems I did :)

```
$ time python gen_html.py /Users/pybob/code/newbies-part2

HTML pages and index generated in html_pages
python gen_html.py /Users/pybob/code/newbies-part2  0.20s user 0.02s system 85% cpu 0.268 total

$ time cargo run -- --directory /Users/pybob/code/newbies-part2
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/md_to_html --directory /Users/pybob/code/newbies-part2`
HTML pages and index generated in html_pages
cargo run -- --directory /Users/pybob/code/newbies-part2  0.04s user 0.03s system 26% cpu 0.292 total

$ cargo build --release
$ time ./target/release/md_to_html --directory /Users/pybob/code/newbies-part2
HTML pages and index generated in html_pages
./target/release/md_to_html --directory /Users/pybob/code/newbies-part2  0.00s user 0.01s system 79% cpu 0.020 total
```
