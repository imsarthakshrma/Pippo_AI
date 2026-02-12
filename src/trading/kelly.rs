/// Implementation of the Kelly Criterion for optimal position sizing.
pub struct KellySizer;

impl KellySizer {
    /// Calculates the optimal bet size based on the Kelly Criterion.
    ///
    /// formula: `f = (bp - q) / b`
    /// - `b`: decimal odds minus 1
    /// - `p`: model's estimated probability of success
    /// - `q`: probability of failure (1 - p)
    /// 
    /// This implementation includes a `max_fraction` cap to prevent 
    /// excessive risk in high-confidence scenarios.
    pub fn calculate_position_size(balance: f64, odds: f64, model_prob: f64, max_fraction: f64) -> f64 {
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
