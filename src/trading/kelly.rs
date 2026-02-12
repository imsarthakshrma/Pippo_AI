pub struct KellySizer;

impl KellySizer {
    pub fn calculate_position_size(balance: f64, odds: f64, model_prob: f64, max_fraction: f64) -> f64 {
        // f = (bp - q) / b
        // b: odds - 1
        // p: model_prob
        // q: 1 - p
        
        if model_prob <= (1.0 / odds) {
            return 0.0;
        }

        let b = odds - 1.0;
        let p = model_prob;
        let q = 1.0 - p;
        
        let fraction = (b * p - q) / b;
        let sized_fraction = fraction.min(max_fraction);
        
        (balance * sized_fraction).max(0.0)
    }
}
