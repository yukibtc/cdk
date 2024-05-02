pub use bitcoin::amount::Amount;

pub trait SplitAmount: Sized {
    /// Split into parts that are powers of two
    fn split(&self) -> Vec<Self>;
}

impl SplitAmount for Amount {
    fn split(&self) -> Vec<Self> {
        let sats: u64 = self.to_sat();
        (0_u64..64)
            .rev()
            .filter_map(|bit| {
                let part = 1 << bit;
                ((sats & part) == part).then_some(Self::from_sat(part))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_amount() {
        assert_eq!(Amount::from_sat(1).split(), vec![Amount::from_sat(1)]);
        assert_eq!(Amount::from_sat(2).split(), vec![Amount::from_sat(2)]);
        assert_eq!(
            Amount::from_sat(3).split(),
            vec![Amount::from_sat(2), Amount::from_sat(1)]
        );
        let amounts: Vec<Amount> = [8, 2, 1].iter().map(|a| Amount::from_sat(*a)).collect();
        assert_eq!(Amount::from_sat(11).split(), amounts);
        let amounts: Vec<Amount> = [128, 64, 32, 16, 8, 4, 2, 1]
            .iter()
            .map(|a| Amount::from_sat(*a))
            .collect();
        assert_eq!(Amount::from_sat(255).split(), amounts);
    }
}
