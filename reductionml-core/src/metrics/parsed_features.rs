use crate::{
    metrics::Metric, utils::GetInner, ActionProbsPrediction, CBLabel, Features, ScalarPrediction,
    SimpleLabel,
};

use super::MetricValue;

pub struct ParsedFeaturesMetric {
    pub count: u64,
}

impl ParsedFeaturesMetric {
    pub fn new() -> ParsedFeaturesMetric {
        ParsedFeaturesMetric { count: 0 }
    }
}

impl Metric for ParsedFeaturesMetric {
    fn add_point(
        &mut self,
        features: &Features,
        label: &crate::types::Label,
        prediction: &crate::types::Prediction,
    ) {
        match features {
            Features::SparseSimple(s) => self.count += s.all_features().count() as u64,
            Features::SparseSimpleRef(s) => self.count += s.all_features().count() as u64,
            Features::SparseCBAdf(feats) => {
                self.count += feats
                    .shared
                    .as_ref()
                    .map(|x| x.all_features().count())
                    .unwrap_or(0) as u64;
                self.count += feats
                    .actions
                    .iter()
                    .map(|x| x.all_features().count())
                    .sum::<usize>() as u64;
            }
            Features::SparseCBAdfRef(feats) => {
                self.count += feats
                    .shared
                    .as_ref()
                    .map(|x| x.all_features().count())
                    .unwrap_or(0) as u64;
                self.count += feats
                    .actions
                    .iter()
                    .map(|x| x.all_features().count())
                    .sum::<usize>() as u64;
            }
        }
    }

    fn get_value(&self) -> MetricValue {
        MetricValue::Int(self.count.try_into().unwrap())
    }

    fn get_name(&self) -> String {
        "Parsed features".to_owned()
    }
}
