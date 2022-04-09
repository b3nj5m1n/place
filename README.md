# r/place dataset parser

Parses the r/place dataset from 2017 and 2022 into a sqlite database.

## Obtaining the datasets

### 2017

You can find the original reddit post [here](https://www.reddit.com/r/redditdata/comments/6640ru/place_datasets_april_fools_2017/).

The full dataset is located [here](https://storage.googleapis.com/place_data_share/place_tiles.csv).

### 2022

You can find the original reddit post [here](https://www.reddit.com/r/place/comments/txvk2d/rplace_datasets_april_fools_2022/).

The full dataset is located [here](https://placedata.reddit.com/data/canvas-history/2022_place_canvas_history.csv.gzip).

## Compiling

Compiling should be as easy as running

```bash
cargo build --release
```

You can then run the executable like this

```bash
./target/release/place
```

## Running

The program can be invoked with the `-d` flag to specify the database to write to. When no database is specified, the program runs in dry mode, parsing the entries but not doing anything with them.

You can pipe data to the program, or specify files to read as arguments.

To parse the first 100k entries from 2017 dataset:

```bash
head -n 100000 ./place_tiles.csv | ./target/release/place -d test.sqlite
```

To parse the full datasets from 2017 and 2022:

```bash
./target/release/place ./place_tiles.csv ./2022_place_canvas_history.csv -d test.sqlite
```
