# Group 1: Sentiment Analysis in Rust

## Group Members
* Hassam Uddin: hassamu2
* Liza George: lizag2
* Aydan Pirani: apirani2

## Introduction
We are developing a sentiment analysis tool that works on a variety of mediums. Our tool should be able to analyze and display the emotional sentiment of any textual time series. 
We will create various drivers to hook our tool into different applications and APIs, such as Twitter, Reddit, SMS, etc. We chose to do this project because it is practical, 
reasonable to do in the timeframe we have, and allows us to explore our interests while still experimenting with Rust and its capabilities. 

## System Overview 
The first thing we will need to do is create a Rust library that allows us to feed in a variable length time-series and uses the MapReduce paradigm to calculate the sentiment of 
the text. We will initially 
make use of a crate, such as [sentiment](https://crates.io/crates/sentiment), but if time permits, we could attempt to implement our own sentiment analysis algorithm. Then, we can
asynchronously work on a variety of drivers for this library. For example, we could make use of the Twitter API and a [twitter crate](https://github.com/egg-mode-rs/egg-mode), 
or we could work with the Reddit API with a crate like [roux](https://crates.io/crates/roux), or we could work directly with some file format, such as a CSV using a 
[parsing crate](https://crates.io/crates/csv). Once we've calculated sentiment, we can perform a variety of analyses on it and graph them. 
We will finalize a proper list of mediums that we would like to support after looking deeper at the libraries and tooling available for Rust.

Our first milestone will be to make use of a sentiment analysis crate, and MapReduce, to calculate the sentiment of long series of text. After that, we can begin working on an MVP
that reads text from a file and calculates sentiment on that. Once this library and driver is complete, we will split up and attempt to implement different drivers, 
making use of crates and various APIs. 

## Possible Challenges
Our biggest concern is teamwork. None of us have collaborated on a project for a serious grade before while making use of version control and keeping to best practices. 
Although we are familiar with Git and can theoretically work with it, actually doing so will require a learning curve. 
