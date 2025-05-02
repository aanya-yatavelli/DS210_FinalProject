//building feature vectors (numeric, one-hot, tags)
use std::collections::{HashMap, HashSet};

pub struct FeatureBuilder {
    numeric_fields: Vec<String>,
    categorical_values: HashMap<String, Vec<String>>,
    tag_list: Vec<String>,
}

impl FeatureBuilder {
    // initialize builder by scanning records for categories and tags
    pub fn new(records: &[HashMap<String, String>]) -> Self{
        let numeric_fields = vec![
        "age", "app_usage_time_min", "swipe_right_ratio", "likes_received",
        "mutual_matches", "profile_pics_count", "bio_length", "message_sent_count",
        "emoji_usage_rate", "last_active_hour", "height_cm", "weight_kg",
    ].into_iter().map(String::from).collect();

        let categorical_fields = vec![
            "gender", "sexual_orientation", "location_type", "income_bracket",
            "education_level", "app_usage_time_label", "swipe_right_label",
            "swipe_time_of_day", "match_outcome", "zodiac_sign", "body_type",
            "relationship_intent",];

        // unique values per categorical field
        let mut categorical_values = HashMap::new();
        for &field in &categorical_fields {
            let mut set = HashSet::new();
            for rec in records {
                if let Some(val) = rec.get(field){
                    set.insert(val.clone());
                }
            }

            let mut vec: Vec<String> = set.into_iter().collect();
            vec.sort();
            categorical_values.insert(field.to_string(), vec);
        }

        // building tag lsit from interest_tags column
        let mut tag_set = HashSet::new();
        for rec in records {
            if let Some(tags) = rec.get("interest_tags"){
                for tag in tags.split(','){
                    tag_set.insert(tag.trim().to_string());
                }
            }
        }

        let mut tag_list: Vec<String> = tag_set.into_iter().collect();
        tag_list.sort();

        FeatureBuilder{
            numeric_fields, categorical_values, tag_list,}
        }
        
        // total dimensions of each feature vector
        pub fn dimension(&self) -> usize{
            let num = self.numeric_fields.len();
            let cat: usize = self.categorical_values.values().map(|v| v.len()).sum();
            let tags = self.tag_list.len();
            num + cat + tags
        }

    /// vectorize records into vector of vectors
    pub fn vectorize(&self, records: &[HashMap<String, String>]) -> Vec<Vec<f64>> {
        records.iter().map(|rec| {
            let mut v = Vec::new();
            // numeric
            for key in &self.numeric_fields {
                let val = rec.get(key).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                v.push(val);
            }
            // categorical one-hot
            for (key, vals) in &self.categorical_values {
                let entry = rec.get(key).cloned().unwrap_or_default();
                for option in vals {
                    v.push((option == &entry) as u8 as f64);
                }
            }
            // tags
            let rec_tags: HashSet<&str> = rec.get("interest_tags").map_or(HashSet::new(), |s|
                s.split(',').map(|t| t.trim()).collect()
            );
            for tag in &self.tag_list {
                v.push(rec_tags.contains(tag.as_str()) as u8 as f64);
            }
            v
        }).collect()
    }

    // ordered list of numeric field names
    pub fn numeric_fields(&self) -> &Vec<String> {
        &self.numeric_fields
    }

    /// getting the mapping of categorical field to ordered options
    pub fn categorical_values(&self) -> &HashMap<String, Vec<String>> {
        &self.categorical_values
    }

    /// get the ordered list of all tag names
    pub fn tag_list(&self) -> &Vec<String> {
        &self.tag_list
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_feature_dimension_and_vector() {
        //  single-record dataset with minimal fields
        let mut rec = HashMap::new();
        rec.insert("age".to_string(), "30".to_string());
        rec.insert("gender".to_string(), "Other".to_string());
        rec.insert("interest_tags".to_string(), "x,y".to_string());

        // initialize builder on this one record
        let builder = FeatureBuilder::new(&[rec.clone()]);
        let dim = builder.dimension();
        let vecs = builder.vectorize(&[rec]);

        // expect one feature vector of length == dimension
        assert_eq!(vecs.len(), 1);
        assert_eq!(vecs[0].len(), dim);              

        // first element (age) should be parsed to 30.0
        assert!((vecs[0][0] - 30.0).abs() < 1e-6);    
    }
}