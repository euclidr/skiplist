
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand;

pub const DEFAULT_LEVELS: usize = 32;
pub const DEFAULT_PROPABILITY: f64 = 0.5;

pub struct LevelGenerator {
    p: f64,
    levels: usize,
    cur_level_limit: usize,
    rng: StdRng,
}

impl LevelGenerator {

    pub fn new() -> Self {
        Self::with_config(DEFAULT_PROPABILITY, DEFAULT_LEVELS)
    }

    pub fn with_propability(p: f64) -> Self {
        Self::with_config(p, DEFAULT_LEVELS)
    }

    pub fn with_config(p: f64, levels: usize) -> Self {
        Self {
            p,
            levels,
            cur_level_limit: 0,
            rng: StdRng::from_entropy(),
        }
    }

    /// choose a level
    /// 
    /// # Examples
    /// 
    /// ```
    /// use skiplist::level_generator::LevelGenerator;
    /// 
    /// let mut lg = LevelGenerator::new();
    /// lg.choose();
    /// ```
    pub fn choose(&mut self) -> usize {
        let sample: f64 = self.rng.gen();
        let mut level = 0;
        let mut p = self.p;

        while sample < p && level < self.cur_level_limit {
            level += 1;
            p = p*p;
        }

        if level == self.cur_level_limit {
            if level >= self.levels {
                level = self.levels - 1
            }
            self.cur_level_limit = level+1;
        }

        level
    }

    pub fn shrink(&mut self) -> usize {
        if self.cur_level_limit > 0 {
            self.cur_level_limit -= 1;
        }
        self.cur_level_limit
    }
}

impl Clone for LevelGenerator {
    fn clone(&self) -> Self {
        Self::with_config(self.p, self.levels)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn choose() {
        let mut lg = LevelGenerator::new();
        assert_eq!(lg.choose(), 0);
    }
}