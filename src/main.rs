#![feature(map_first_last)]
#![feature(bool_to_option)]

use {
    rayon::prelude::*,
    serde::Deserialize,
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BinaryHeap},
        fs::File,
        io::{stdin, stdout, BufReader, BufWriter, Read, Write},
        time::SystemTime,
    },
};

#[derive(Deserialize)]
struct Tft {
    sets: BTreeMap<String, Set>,
}

#[derive(Deserialize)]
struct Set {
    champions: Vec<Champion>,
    traits: Vec<Trait>,
}

#[derive(Deserialize)]
struct Champion {
    name: String,
    cost: usize,
    traits: Vec<String>,
}

#[derive(Deserialize)]
struct Trait {
    effects: Vec<Effect>,
    name: String,
}

#[derive(Deserialize)]
struct Effect {
    maxUnits: usize,
    minUnits: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct TeamComp {
    active_traits: usize,
    indices: Vec<usize>,
}

impl TeamComp {
    fn print(self, champs: &[&Champion]) {
        let champ_names: Vec<String> = self
            .indices
            .iter()
            .map(|i| champs[*i].name.clone())
            .collect();
        let mut repr = format!("{}: {:?}", self.active_traits, champ_names)
            .replace('\"', "")
            .replace('[', "")
            .replace('\\', "");
        repr.pop();
        println!("{}", repr);
    }
}

const DEFAULT_TOP_N: usize = 10;

fn main() {
    // Try to load en_us.json
    let tft: Tft = if let Ok(file) = File::open("en_us.json") {
        serde_json::from_reader(BufReader::new(file)).unwrap()
    } else {
        // Download and save if it doesn't exist
        println!("TFT data not found, downloading from Data Dragon");
        let tft_resp = ureq::get("https://raw.communitydragon.org/latest/cdragon/tft/en_us.json")
            .call()
            .unwrap();
        let file = File::create("en_us.json").unwrap();
        let tft_string = tft_resp.into_string().unwrap();
        BufWriter::new(file)
            .write_all(tft_string.as_bytes())
            .unwrap();
        serde_json::from_str(tft_string.as_str()).unwrap()
    };

    let mut input_text = String::new();
    print!("Number of units: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let num_units: usize = input_text.trim().parse().expect("could not parse number");

    print!("Include 5+ costs? [y/N] ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let exclude_5_costs = input_text.trim().to_lowercase().as_str() != "y";

    print!("Calculate unique traits? [y/N] ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let calc_unique_traits = input_text.trim().to_lowercase().as_str() == "y";

    print!("Number of results: [{}] ", DEFAULT_TOP_N);
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let trim = input_text.trim();
    let top_n: usize = if trim.is_empty() {
        DEFAULT_TOP_N
    } else {
        trim.parse().expect("could not parse number")
    };

    // Format data
    let (_, current_set) = tft.sets.last_key_value().unwrap();
    let champs: Vec<&Champion> = current_set
        .champions
        .iter()
        .filter(|champ| !(champ.traits.is_empty() || exclude_5_costs && champ.cost > 4))
        .collect();
    let champ_traits: Vec<Vec<usize>> = champs
        .iter()
        .map(|champ| {
            champ
                .traits
                .iter()
                .map(|champ_trait| {
                    current_set
                        .traits
                        .iter()
                        .position(|tft_trait| tft_trait.name == *champ_trait)
                        .expect("no matching champ trait found")
                })
                .collect()
        })
        .collect();
    let traits = current_set
        .traits
        .iter()
        .map(|tft_trait| {
            tft_trait
                .effects
                .iter()
                .rev()
                .filter_map(|effect| {
                    (calc_unique_traits || effect.maxUnits > 1)
                        .then_some([effect.minUnits, effect.maxUnits])
                })
                .collect::<Vec<[usize; 2]>>()
        })
        .collect::<Vec<Vec<[usize; 2]>>>();

    print!("Force traits (e.g. \"assassin:4, brawler:2\"): ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let trim = input_text.trim();
    let forced_traits: Vec<[usize; 2]> = if trim.is_empty() {
        vec![]
    } else {
        trim.split(',')
            .map(|trait_str| {
                let mut trait_parts = trait_str.split(':');
                // [0] is the trait index, [1] is the minimum required
                let force_trait = trait_parts
                    .next()
                    .expect("trait force missing name")
                    .trim()
                    .to_lowercase();

                [
                    current_set
                        .traits
                        .iter()
                        .position(|tft_trait| tft_trait.name.trim().to_lowercase() == force_trait)
                        .unwrap_or_else(|| panic!("invalid trait name {}", force_trait)),
                    trait_parts
                        .next()
                        .expect("trait missing min number")
                        .trim()
                        .parse()
                        .expect("could not parse number"),
                ]
            })
            .collect()
    };

    print!("Trait bonuses (e.g. \"assassin:2, brawler:1\"): ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let trim = input_text.trim();
    let trait_bonus: Vec<[usize; 2]> = if trim.is_empty() {
        vec![]
    } else {
        trim.split(',')
            .map(|trait_str| {
                let mut trait_parts = trait_str.split(':');
                // [0] is the trait index, [1] is the minimum required
                let force_trait = trait_parts
                    .next()
                    .expect("trait force missing name")
                    .trim()
                    .to_lowercase();

                [
                    current_set
                        .traits
                        .iter()
                        .position(|tft_trait| tft_trait.name.trim().to_lowercase() == force_trait)
                        .unwrap_or_else(|| panic!("invalid trait name {}", force_trait)),
                    trait_parts
                        .next()
                        .expect("trait missing min number")
                        .trim()
                        .parse()
                        .expect("could not parse number"),
                ]
            })
            .collect()
    };

    print!("Force champs (e.g. \"leona, velkoz\"): ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let trim = input_text.trim();

    let forced_champs: Vec<usize> = if trim.is_empty() {
        vec![]
    } else {
        trim.split(',')
            .map(|champ_force| {
                champs
                    .iter()
                    .position(|champ| {
                        champ.name.to_lowercase() == champ_force.trim().to_lowercase().as_str()
                    })
                    .expect("invalid champ name")
            })
            .collect()
    };

    // Log num comps and threads
    let num_champs = champs.len();
    let num_combinations = n_choose_k(num_champs, num_units);
    let num_chunks = num_champs - num_units + 1;
    print!(
        "Generating and analyzing {} comps based on {} champs and {} traits using {} threads\nForcing {} champs and {} traits with {} bonuses\n+{}+\n|",
        num_combinations,
        num_champs,
        traits.len(),
        rayon::current_num_threads(),
        forced_champs.len(),
        forced_traits.len(),
        trait_bonus.len(),
        "-".repeat(num_chunks),
    );
    stdout().flush().unwrap();
    let time_start = SystemTime::now();

    let mut chunks_top_n_comps = Vec::new();
    (0..num_chunks)
        .into_par_iter()
        .map(|init_index| {
            // Combination generation boiler plate
            let k_sub_1 = num_units - 1;
            let n_sub_k = num_champs - num_units;
            let mut indices: Vec<usize> = (init_index..init_index + num_units).collect();
            let mut min_heap: BinaryHeap<Reverse<TeamComp>> = BinaryHeap::with_capacity(top_n);
            let mut trait_tally_base = vec![0usize; traits.len()];
            for [trait_idx, bonus] in &trait_bonus {
                trait_tally_base[*trait_idx] += bonus;
            }
            let mut trait_tally = trait_tally_base.clone();

            while indices[0] == init_index && indices[k_sub_1] < num_champs {
                // HOT PATH, 99% OF RUNTIME! Calculate the amount of active traits 
                trait_tally.clone_from(&trait_tally_base);
                let mut active_traits = 0usize;
                if forced_champs.iter().all(|champ| indices.contains(champ)) {
                    for &champ_index in &indices {
                        for trait_idx in &champ_traits[champ_index] {
                            trait_tally[*trait_idx] += 1;
                        }
                    }
                    if forced_traits
                        .iter()
                        .all(|[trait_idx, min_tally]| min_tally <= &trait_tally[*trait_idx])
                    {
                        for trait_idx in 0..traits.len() {
                            for effect in &traits[trait_idx] {
                                if effect[0] <= trait_tally[trait_idx]
                                    && trait_tally[trait_idx] <= effect[1]
                                {
                                    active_traits += effect[0];
                                    break;
                                }
                            }
                        }
                    }
                }

                // Add comp to top N
                let min = min_heap.peek().map_or(0, |m| m.0.active_traits);
                if min_heap.len() == top_n {
                    if active_traits > min {
                        min_heap.pop();
                        min_heap.push(Reverse(TeamComp {
                            active_traits,
                            indices: indices.clone(),
                        }));
                    }
                } else if active_traits >= min {
                    min_heap.push(Reverse(TeamComp {
                        active_traits,
                        indices: indices.clone(),
                    }));
                }

                // Generate the next comp
                let mut t = k_sub_1;
                while t != 0 && indices[t] == n_sub_k + t {
                    t -= 1;
                }
                indices[t] += 1;
                for j in t + 1..num_units {
                    indices[j] = indices[j - 1] + 1;
                }
            }
            print!("#");
            stdout().flush().unwrap();
            min_heap.into_vec()
        })
        .collect_into_vec(&mut chunks_top_n_comps);

    // Combine and distill top N from chunks
    let mut top_n_comps = chunks_top_n_comps.concat();
    top_n_comps.sort_unstable();
    top_n_comps.truncate(top_n);

    println!("|\nCompleted in {:?}", time_start.elapsed().unwrap());
    for comp_traits in top_n_comps {
        comp_traits.0.print(&champs);
    }
    print!("Press enter to continue...");
    stdout().flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

fn n_choose_k(n: usize, k: usize) -> usize {
    let mut combs = 1;
    for i in 0..k {
        combs = (combs * (n - i)) / (i + 1);
    }
    combs
}
