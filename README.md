![](https://github.com/wigasper/antsnap/workflows/build/badge.svg)

# antsnap

`antsnap` is a method for detecting epistatic interactions in genome-wide
association study data. The project builds upon prior methods that used
ant colony optimization to identify epistatic interactions but, in contrast,
attempts to identify 3-SNP interactions. The results are sometimes good, but
ultimately trying to search such a large space is quite difficult and 
may not be an effective substitute for more exhaustive search methods. There
is of course also the potential that this method could be significantly 
improved.

Currently this is written for use with GAMETES 2.0 simulated GWAS data,
which is formulated like so:

```
N1  N2  N2  MOP1    MOP2    MOP3    Class
0   2   1   2   2   2   0
0   2   0   1   0   2   1
```

where the last column is a binary value describing the presence or 
absence of a phenotype.

## Building

```Rust
$ cargo build --release
```

## Usage

Parameters are specified in a config file specified by the `-c` parameter.

```
$ target/release/antsnap -c config.toml
```

