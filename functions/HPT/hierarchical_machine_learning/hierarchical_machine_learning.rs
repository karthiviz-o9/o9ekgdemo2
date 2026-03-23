use std::collections::HashMap;
use std::sync::Arc;
use ndarray::{Array1, Array2};
use hnsw_rs::prelude::*;

pub struct Actuals {
    pub item_id: String,
    pub location_id: String,
    pub values: Vec<f32>,
}

pub struct Item {
    pub id: String,
    pub category_embedding: Vec<f32>,
}

pub struct Location {
    pub id: String,
    pub region_embedding: Vec<f32>,
}

pub struct Demand {
    pub item_id: String,
    pub location_id: String,
    pub forecast: Vec<f32>,
}

pub struct DemandForecaster {
    item_embeddings: HashMap<String, Vec<f32>>,
    location_embeddings: HashMap<String, Vec<f32>>,
    history: HashMap<(String, String), Vec<f32>>,
    hnsw: Hnsw<f32, DistCosine>,
}

impl DemandForecaster {
    pub fn new(items: Vec<Item>, locations: Vec<Location>) -> Self {
        let mut item_map = HashMap::new();
        let mut loc_map = HashMap::new();
        let mut data = Vec::new();

        for item in items {
            item_map.insert(item.id.clone(), item.category_embedding.clone());
            data.push(item.category_embedding);
        }
        
        let hnsw = Hnsw::new(16, data.len(), 32, 200, DistCosine {});
        
        Self {
            item_embeddings: item_map,
            location_embeddings: loc_map,
            history: HashMap::new(),
            hnsw,
        }
    }

    pub fn ingest_actuals(&mut self, actuals: Vec<Actuals>) {
        for a in actuals {
            self.history.insert((a.item_id, a.location_id), a.values);
        }
    }

    pub fn forecast(&self, item_id: &str, location_id: &str) -> Demand {
        let target_embedding = self.item_embeddings.get(item_id)
            .cloned()
            .unwrap_or_else(|| vec![0.0; 32]);

        let neighbors = self.hnsw.search(&target_embedding, 5, 20);
        
        let mut weighted_sum = Array1::zeros(12);
        let mut total_weight = 0.0;

        for neighbor in neighbors {
            let weight = 1.0 / (neighbor.distance + 1e-6);
            if let Some(hist) = self.history.values().next() {
                let hist_arr = Array1::from_vec(hist.clone());
                weighted_sum += &(hist_arr * weight);
                total_weight += weight;
            }
        }

        Demand {
            item_id: item_id.to_string(),
            location_id: location_id.to_string(),
            forecast: (weighted_sum / total_weight).to_vec(),
        }
    }
}