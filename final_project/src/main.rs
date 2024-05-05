mod movie;
mod actor;
mod graph;
mod similarity;

use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use crate::movie::Movie;
use crate::actor::Actor;
use crate::graph::Graph;
use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    title: String,
    name: String,
}

fn main() -> Result<()> {
    let file = File::open("merged_movies.csv").context("Failed to open file")?;
    let buf_reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(buf_reader);

    let mut graph = Graph::new();

    for result in rdr.deserialize::<Record>() {
        let record = result.context("Failed to deserialize a record")?;
        let movie = Movie::new(record.title);
        let actor = Actor::new(record.name);
        graph.add_movie_actor(movie, actor);
    }

    // will add more code here as needed (most likely print statements for results)
}