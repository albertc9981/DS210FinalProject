use std::collections::{HashMap, HashSet};
use crate::movie::Movie;
use crate::actor::Actor;
use crate::similarity::calculate_jaccard_similarity;

pub struct Graph {
    pub movies: HashMap<Movie, HashSet<Actor>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            movies: HashMap::new(),
        }
    }

    pub fn add_movie_actor(&mut self, movie: Movie, actor: Actor) {
        self.movies.entry(movie).or_insert_with(HashSet::new).insert(actor);
    }

    pub fn contains_movie(&self, movie: &Movie) -> bool {
        self.movies.contains_key(movie)
    }

    pub fn contains_actor_in_movie(&self, movie: &Movie, actor: &Actor) -> bool {
        self.movies.get(movie).map_or(false, |actors| actors.contains(actor))
    }
    
    pub fn find_most_similar_movies(&self) -> ((Movie, Movie), f64) {
        let mut max_similarity = 0.0;
        let mut most_similar_pair = (Movie::new("".to_string()), Movie::new("".to_string()));

        let movie_list: Vec<_> = self.movies.keys().collect();

        for i in 0..movie_list.len() {
            for j in i + 1..movie_list.len() {
                let set1 = &self.movies[movie_list[i]];
                let set2 = &self.movies[movie_list[j]];
                let similarity = calculate_jaccard_similarity(set1, set2);
                if similarity > max_similarity {
                    max_similarity = similarity;
                    most_similar_pair = (movie_list[i].clone(), movie_list[j].clone());
                }
            }
        }

        (most_similar_pair, max_similarity)
    }

    pub fn find_least_similar_movies(&self) -> ((Movie, Movie), f64) {
        let mut min_similarity = 1.0;
        let mut least_similar_pair = (Movie::new("".to_string()), Movie::new("".to_string()));

        let movie_list: Vec<_> = self.movies.keys().collect();

        for i in 0..movie_list.len() {
            for j in i + 1..movie_list.len() {
                let set1 = &self.movies[movie_list[i]];
                let set2 = &self.movies[movie_list[j]];
                let similarity = calculate_jaccard_similarity(set1, set2);
                if similarity < min_similarity {
                    min_similarity = similarity;
                    least_similar_pair = (movie_list[i].clone(), movie_list[j].clone());
                }
            }
        }

        (least_similar_pair, min_similarity)
    }
}