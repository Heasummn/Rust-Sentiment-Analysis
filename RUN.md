# RUN.md

## Setup And Compiling
In order to build this project, you need to make sure you have Rust installed on your computer. If you do not already have it, use the instruction in [this](https://www.rust-lang.org/tools/install) to get it.

To create and run on your machine, clone the project to your machine and call `cargo clean` and `cargo run` in your terminal. 
```console
$ git clone https://github.com/Heasummn/cs128-honors
$ cargo clean
$ cargo run
```

## Running
You will also need to fill in the `.env` file with your Twitter API credentials. For more detailed instructions on how to access the twitter API, check out [this](https://developer.twitter.com/en/docs/twitter-api/getting-started/getting-access-to-the-twitter-api) document.

To use the interface, you can chose between analysing the sentiment of a provided CSV file or of the tweets of a provided user. Use arrow keys and the Enter key to pick between the two options in your terminal. Make sure that the CSV file you enter has two columns, one labeled 'text' and one labeled 'date'. Each row can be a string of text and a timestamp in UTC format (like `YYYY-DD-MM HH:MM:SS`). You can see examples of CSV files like this in the `data` directory.
