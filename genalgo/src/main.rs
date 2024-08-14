use core::fmt;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

struct Individual {
    score: i32,
    gene: i32,
}

impl Individual {
    fn calc_score(&mut self) {
        self.score = self.gene * self.gene;
    }

    fn crossover(&self, other: &Individual, rng: &mut impl Rng) -> Individual {
        let max_gene = self.gene.max(other.gene);
        let num_bits = 32 - max_gene.leading_zeros();

        let split_index = rng.gen_range(0..num_bits);
        let mask = (1 << split_index) - 1;
        // println!("Crossover gene1: {:#032b} gene2: {:#032b} with mask: {:#032b} split index: {}", self.gene, other.gene, mask, split_index);
        // println!("{:#032b} OR {:#032b}", self.gene & mask, other.gene & !mask);
        let gene = (self.gene & mask) | (other.gene & !mask);
        // println!("Crossover gene: {:#032b}", gene);

        Individual { gene, score: 0 }
    }

    fn mutate(&mut self, rng: &mut impl Rng) {
        let num_bits = 32 - self.gene.leading_zeros();
        let mask = 1 << rng.gen_range(0..num_bits);
        // println!("Mutating gene: {:#032b} with mask: {:#032b}", self.gene, mask);
        self.gene ^= mask;
        // println!("Mutated gene: {:#032b}", self.gene);
    }
}

impl fmt::Debug for Individual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.gene)
    }
}

fn main() {
    let num_iter = 1000;
    let pop_size = 5;
    let mut_prob = 0.1;
    let mut pop: Vec<Individual> = Vec::with_capacity(pop_size);

    let mut rng = rand::thread_rng();
    for _ in 0..pop_size {
        let gene = rng.gen_range(0..=32);
        pop.push(Individual { gene, score: 0 });
    }

    println!("Initial population: {:?}", pop);
    let mut total_score = calc_fitness(&mut pop);

    for i in 0..num_iter {
        let probs: Vec<f32> = pop.iter()
        .map(|ind| ind.score as f32 / total_score as f32)
        .collect();
    
        let dist = WeightedIndex::new(&probs).unwrap();
        pop = breed(pop, pop_size, mut_prob, &dist, &mut rng);
        total_score = calc_fitness(&mut pop);

        println!("Population after generation {}: {:?}", i, pop);
    }
}

fn calc_fitness(pop: &mut Vec<Individual>) -> i32 {
    let mut total_score = 0;
    for individual in pop.iter_mut() {
        individual.calc_score();
        total_score += individual.score;
    }

    total_score
}

fn breed(pop: Vec<Individual>, num_children: usize, mut_prob: f32, dist: &WeightedIndex<f32>, rng: &mut ThreadRng) -> Vec<Individual> {
    let mut children = Vec::with_capacity(num_children as usize);
    for _ in 0..num_children {
        let parent1_index = dist.sample(rng);
        let parent2_index = dist.sample(rng);

        let parent1 = &pop[parent1_index];
        let parent2 = &pop[parent2_index];

        let mut child = parent1.crossover(parent2, rng);
        if rng.gen_range(0.0..1.0) < mut_prob {
            child.mutate(rng);
        }
        children.push(child);
    }

    children
}
