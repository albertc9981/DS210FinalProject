# DS210FinalProject

Project Summary: This is the final project for the DS210 Spring 2024 semester.

Instructions on Navigating the Repository:
- titles.csv: 1 of the 2 .csv files from Kaggle
- credits.csv: 1 of the 2 .csv files from Kaggle
- mergingcsv.ipynb: a python file I used to merge the 2 .csv files into one to make it easier to work with.
- final_project Folder:
  - Cargo.lock: the Cargo.lock file initialized with the "cargo new" command
  - Cargo.toml: the Cargo.toml file initialized with the "cargo new" command. Contains updated deliverables needed for my project.
  - merged_movies.csv: the merged version of my 2 .csv files
  - writeup.pdf: the write up for my final project
      - src Folder:
        - actor.rs: keeps track of actor names from the .csv file to use in other modules
        - graph.rs: contains all code that handles making the graph and finding similarities between nodes using the similarity.rs module
        - main.rs: handles opening and accessing the .csv file, printing outputs, test cases.
        - movie.rs: keeps track of movie titles from the .csv file to use in other modules
        - similarity.rs: calculates similarity for the friends of friends project using Jaccard similarity.
