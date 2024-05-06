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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn movie_creation() {
        let movie = Movie::new("Inception".to_string());
        assert_eq!(movie.title, "Inception");
    }

    #[test]
    fn actor_creation() {
        let actor = Actor::new("Leonardo DiCaprio".to_string());
        assert_eq!(actor.name, "Leonardo DiCaprio");
    }

    #[test]
    fn add_movie_actor() {
        let mut graph = Graph::new();
        let movie = Movie::new("Inception".to_string());
        let actor = Actor::new("Leonardo DiCaprio".to_string());
        graph.add_movie_actor(movie.clone(), actor.clone());
        assert!(graph.contains_movie(&movie));
        assert!(graph.contains_actor_in_movie(&movie, &actor));
    }

    #[test]
    fn test_jaccard_similarity() {
        let mut set1 = HashSet::new();
        set1.insert(Actor::new("Actor A".to_string()));
        set1.insert(Actor::new("Actor B".to_string()));

        let mut set2 = HashSet::new();
        set2.insert(Actor::new("Actor A".to_string()));
        set2.insert(Actor::new("Actor C".to_string()));

        let similarity = calculate_jaccard_similarity(&set1, &set2);
        let expected_similarity = 1.0 / 3.0; // More explicit expected value
        let tolerance = 0.01; // Use a more reasonable tolerance for this comparison

        assert!((similarity - expected_similarity).abs() < tolerance, "Expected similarity close to {}, but got {}", expected_similarity, similarity);
    }
}