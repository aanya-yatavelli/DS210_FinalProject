// CLI for K-Means Clustering (K-MC)

use clap::Parser;
use log::{info};
use env_logger::Env;
use std::error::Error;

mod data;
mod features;
mod clustering;
mod analytics;

//CLI arguements
#[derive(Parser, Debug)]
#[command(name = "kmeans_cluster", version, about = "K-neans Clustering on Dating app data")]
struct Cli{
    // input csv with use attributes
    #[arg(short, long, default_value = "dating_app_behavior_dataset_extended1.csv")]
    input: String,

    //number of clusters k
    #[arg(short = 'k', long, default_value_t = 5)]
    clusters: usize,

    // max iterations for K-MC
    #[arg(short = 'm', long, default_value_t = 100)]
    max_iter: usize,

    // max processed records
    #[arg(short = 's', long, default_value_t = 1000)]
    sample: usize,

    // output CSV of cluster assignment
    #[arg(short,long)]
    output: Option<String>,

    //verbose logging
    #[arg(short,long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    println!("\nK-means Clustering on Dating App Data\n");

    // initialize logging
    let env = Env::default().filter_or("LOG_LEVEL", if args.verbose { "info" } else { "warn" });
    env_logger::init_from_env(env);
    info!("Verbose logging enabled");

    //load records and sample
    info!("Loading data from '{}'...", &args.input);
    let mut records = data::load_records(&args.input)?;
    info!("Loaded {} records", records.len());
    if records.len() > args.sample {
        info!("Sampling {} records", args.sample);
        records = data::sample_records(&records, args.sample);
    }
    info!("Using {} records", records.len());

    //biuld feature vectors
    info!("extracting features...");
    let builder = features::FeatureBuilder::new(&records);
    let vectors = builder.vectorize(&records);
    info!("feature dimension: {}", builder.dimension());

    // run K-MC
    info!("clustering into {} clusters (max {} iterations...)", args.clusters, args.max_iter);
    let (centroids, assignments) = clustering::kmeans(&vectors, args.clusters, args.max_iter);
    info!("Clustering complete");

    // report cluster sizes
    let sizes = analytics::cluster_sizes(&assignments, args.clusters);
    println!("cluster sizes:");
    for (i,size) in sizes.iter().enumerate() {
        println!(" Cluster {}: {} records", i, size)
    }

    // reconstruct the feature names 
    let mut feature_names = Vec::new();

    // numeric fields
    feature_names.extend(builder.numeric_fields().iter().cloned());  

    // one-hot categorical fields
    for (field, options) in builder.categorical_values() {                       // EDIT
        for option in options {
            feature_names.push(format!("{}={}", field, option));
        }
    }

    // tag binaries
    for tag in builder.tag_list() {
        feature_names.push(format!("tag:{}", tag));
    }

    //  show the top 5 strongest features for each centroid
    println!("\nCluster centroids (top 5 features):");
    for (ci, cent) in centroids.iter().enumerate() {
        let mut pairs: Vec<(&String, &f64)> = feature_names.iter().zip(cent.iter()).collect();
        pairs.sort_by(|a,b| b.1.partial_cmp(a.1).unwrap());
        println!(" Cluster {}:", ci);
        for (name, &val) in pairs.iter().take(5) {
            println!("   • {:<30} {:.3}", name, val);
        }
    }

    // write assignments
    if let Some(path) = args.output {
        info!("Writing assignments to '{}'", path);
        analytics::write_assignments_csv(&assignments, path.as_str())?;
        println!("Assignments saved to {}", path);
    }
    Ok(())
}