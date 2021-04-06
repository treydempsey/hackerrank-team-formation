# Team Formation

This is a solution I crafted to the Team Formation problem from Hacker Rank.

My initial solution is in solution.rb and is the one I submitted. During testing
I had issues with test cases 10 through 15 timing out. Although my solution is
a relatively stright forward brute force solution, I felt that it would excute
within time limits if written in a more performant language.

Thus the rust solution is basically a one to one port of the ruby code and does
in fact show that it can complete in reasonable time, including for large
solutions of 2.7MM scores (input100.txt).

## Building

The ruby solution should run as is without any external dependencies beyond a
reasonably modern version of ruby.

The rust code will build with cargo and also has no external crate dependencies.

```bash
cargo build --release
```

The debug version of the rust code is significantly slower than the release version
due to bounds checks and other checks not typically present in a release build.

## Running

The examples follow the same structure as the hackerrank execution requirements.
Their ruby template code assumes an environment variable, OUTPUT\_PATH, is set,
writing resulting data to that file. Input is read from the standard in.

The rust code foregoes OUTPUT\_PATH and reads and writes to standard in and
standard out respectively.

```bash
OUTPUT_PATH=output.txt ruby solution.rb < input001.txt
```

```bash
./target/release/team-formation < input001.txt
```
