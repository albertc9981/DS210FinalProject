mod movie;
mod actor;
mod graph;
mod similarity;

use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use movie::Movie;
use actor::Actor;
use graph::Graph;
use similarity::calculate_jaccard_similarity;
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

    let (most_similar, similarity_score) = graph.find_most_similar_movies();
    let (least_similar, dissimilarity_score) = graph.find_least_similar_movies();

    println!("Most Similar Movies: {:?} with similarity score: {}", most_similar, similarity_score);
    println!("Least Similar Movies: {:?} with dissimilarity score: {}", least_similar, dissimilarity_score);

    Ok(())
}