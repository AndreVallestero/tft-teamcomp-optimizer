use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt,
    io::{stdin, stdout, Read, Write},
    time::SystemTime,
    ops::Index
};
use rayon::prelude::*;
use Synergy::*;

// TODO generate all data up to core code using a macro or build.rs
//      use manifest file from datadragon
const SYNERGIES: usize = 27; 
const NUM_CHAMPS: usize = 58; 
const NUM_CHAMPS_NO_FIVE: usize = 50;

#[derive(Copy, Clone)]
enum Synergy {
    Abomination,
    Coven,
    Dawnbringer,
    Draconic,
    Dragonslayer,
    Eternal,
    Forgotten,
    Hellion,
    Ironclad,
    Nightbringer,
    Redeemed,
    Revenant,
    Verdant,
    
    Assassin,
    Brawler,
    Caretaker,
    Cavalier,
    Cruel,
    Godking,
    Invoker,
    Knight,
    Legionnaire,
    Mystic,
    Ranger,
    Renewer,
    Skirmisher,
    Spellweaver,
}
const CHAMPS: [(&str, &[Synergy]); NUM_CHAMPS] = [
    ("aatrox", &[Redeemed, Legionnaire]),
    ("aphelios", &[Nightbringer, Ranger]),
    ("ashe", &[Verdant, Draconic, Ranger]),
    ("brand", &[Abomination, Spellweaver]),
    ("diana", &[Nightbringer, Dragonslayer, Assassin]),
    ("draven", &[Forgotten, Legionnaire]),
    ("gragas", &[Dawnbringer, Brawler]),
    ("hecarim", &[Forgotten, Cavalier]),
    ("ivern", &[Revenant, Invoker, Renewer]),
    ("jax", &[Ironclad, Skirmisher]),
    ("kalista", &[Abomination, Legionnaire]),
    ("karma", &[Dawnbringer, Invoker]),
    ("katarina", &[Forgotten, Assassin]),
    ("kennen", &[Hellion, Skirmisher]),
    ("khazix", &[Dawnbringer, Assassin]),
    ("kled", &[Hellion, Cavalier]),
    ("leblanc", &[Coven, Assassin]),
    ("leesin", &[Nightbringer, Skirmisher]),
    ("leona", &[Redeemed, Knight]),
    ("lissandra", &[Coven, Renewer]),
    ("lulu", &[Hellion, Mystic]),
    ("lux", &[Redeemed, Mystic]),
    ("mordekaiser", &[Dragonslayer, Legionnaire]),
    ("morgana", &[Coven, Nightbringer, Mystic]),
    ("nautilus", &[Ironclad, Knight]),
    ("nidalee", &[Dawnbringer, Skirmisher]),
    ("nocturne", &[Revenant, Assassin]),
    ("nunu", &[Abomination, Brawler]),
    ("pantheon", &[Dragonslayer, Skirmisher]),
    ("poppy", &[Hellion, Knight]),
    ("rell", &[Redeemed, Ironclad, Cavalier]),
    ("riven", &[Dawnbringer, Legionnaire]),
    ("ryze", &[Abomination, Forgotten, Mystic]),
    ("sejuani", &[Nightbringer, Cavalier]),
    ("sett", &[Draconic, Brawler]),
    ("soraka", &[Dawnbringer, Renewer]),
    ("syndra", &[Redeemed, Invoker]),
    ("taric", &[Verdant, Knight]),
    ("thresh", &[Forgotten, Knight]),
    ("trundle", &[Dragonslayer, Skirmisher]),
    ("udyr", &[Draconic, Skirmisher]),
    ("varus", &[Redeemed, Ranger]),
    ("vayne", &[Forgotten, Ranger]),
    ("velkoz", &[Redeemed, Spellweaver]),
    ("viktor", &[Forgotten, Spellweaver]),
    ("vladmir", &[Nightbringer, Renewer]),
    ("warwick", &[Forgotten, Brawler]),
    ("yasuo", &[Nightbringer, Legionnaire]),
    ("ziggs", &[Hellion, Spellweaver]),
    ("zyra", &[Draconic, Spellweaver]),
    ("darius", &[Nightbringer, Knight, Godking]),
    ("garen", &[Dawnbringer, Knight, Godking]),
    ("heimerdinger", &[Draconic, Caretaker, Renewer]),
    ("kayle", &[Redeemed, Verdant, Legionnaire]),
    ("kindred", &[Eternal, Mystic, Ranger]),
    ("teemo", &[Hellion, Cruel, Invoker]),
    ("viego", &[Forgotten, Assassin, Skirmisher]),
    ("volibear", &[Revenant, Brawler])
];

fn calc_synergies(
    comp: &Vec<usize>,
    champ_forces: &Vec<usize>,
    calc_unique_synergies: bool,
) -> usize {
    if !champ_forces.iter().all(|champ| comp.contains(champ)) {
        return 0;
    }

    let mut synergy_tally = [0usize; SYNERGIES];
    for champ_index in comp {
        for synergy in CHAMPS[*champ_index].1 {
            synergy_tally[*synergy as usize] += 1;
        }
    }

    let mut active_synergies = 0;
    if synergy_tally[Abomination] >= 3 {
        active_synergies += synergy_tally[Abomination]
    }
    if synergy_tally[Coven] >= 3 {
        active_synergies += 3
    }
    active_synergies += synergy_tally[Dawnbringer] / 2 * 2;
    match synergy_tally[Draconic] {
        3 | 5 => active_synergies += synergy_tally[Draconic],
        _ => (),
    }
    active_synergies += synergy_tally[Dragonslayer] / 2 * 2;
    active_synergies += synergy_tally[Forgotten] / 3 * 3;
    match synergy_tally[Hellion] {
        3 | 5 | 7 => active_synergies += synergy_tally[Hellion],
        _ => (),
    }
    if synergy_tally[Ironclad] >= 2 {
        active_synergies += synergy_tally[Ironclad]
    }
    active_synergies += synergy_tally[Nightbringer] / 2 * 2;
    active_synergies += synergy_tally[Redeemed] / 3 * 3;
    if synergy_tally[Revenant] >= 2 {
        active_synergies += synergy_tally[Revenant]
    }
    if synergy_tally[Verdant] >= 2 {
        active_synergies += synergy_tally[Verdant]
    }
    active_synergies += synergy_tally[Assassin] / 2 * 2;
    active_synergies += synergy_tally[Brawler] / 2 * 2;
    if synergy_tally[Cavalier] >= 2 {
        active_synergies += synergy_tally[Cavalier]
    }
    active_synergies += synergy_tally[Invoker] / 2 * 2;
    active_synergies += synergy_tally[Knight] / 2 * 2;
    active_synergies += synergy_tally[Legionnaire] / 2 * 2;
    active_synergies += synergy_tally[Mystic] / 2 * 2;
    active_synergies += synergy_tally[Ranger] / 2 * 2;
    active_synergies += synergy_tally[Renewer] / 2 * 2;
    active_synergies += synergy_tally[Skirmisher] / 3 * 3;
    active_synergies += synergy_tally[Spellweaver] / 2 * 2;
    if calc_unique_synergies {
        active_synergies += synergy_tally[Eternal];
        active_synergies += synergy_tally[Caretaker];
        active_synergies += synergy_tally[Cruel];
        if synergy_tally[Godking] >= 1 {
            active_synergies += 1
        }
    }
    active_synergies
}

/* ############################################################
 * CORE CODE BELOW!
 * DO NOT EDIT ANYTHING BELOW UNLESS YOU KNOW WHAT YOU'RE DOING
 * ############################################################
 */

const DEFAULT_TOP_N: usize = 10;

impl Index<Synergy> for [usize; SYNERGIES] {
    type Output = usize;
    fn index(&self, index: Synergy) -> &Self::Output {
        &self[index as usize]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct CompSynergy {
    synergy: usize,
    indices: Vec<usize>,
}

impl fmt::Display for CompSynergy {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let champ_names: Vec<&str> = self
            .indices
            .iter()
            .map(|i| return CHAMPS[*i].0)
            .collect();
        formatter.write_str(format!("{}: {:?}", self.synergy, champ_names).as_str())
    }
}

fn main() {
    let mut input_text = String::new();
    print!("Number of units: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let num_units: usize = input_text.trim().parse().expect("could not parse number");

    print!("Include 5 costs? [y/N] ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let num_champs = match input_text.trim().to_lowercase().as_str() {
        "y" => NUM_CHAMPS,
        _ => NUM_CHAMPS_NO_FIVE,
    };

    print!("Calculate unique synergies? [y/N] ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let calc_unique_synergies = input_text.trim().to_lowercase().as_str() == "y";

    print!("Force champs (e.g. \"leona, velkoz\"): ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");

    let trim = input_text.trim();
    let champ_forces: Vec<usize> = if trim.is_empty() {
        vec![]
    } else {
        trim.split(",")
            .map(|champ_force| {
                CHAMPS
                    .iter()
                    .position(|champ| champ.0 == champ_force.trim().to_lowercase().as_str())
                    .expect("invalid champ name")
            })
            .collect()
    };

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

    let time_start = SystemTime::now();

    // Log num comps and threads
    let num_combinations = n_choose_k(num_champs, num_units);
    let num_chunks = num_champs - num_units + 1;
    print!(
        "Generating and analyzing {} comps with {} champs using {} threads\n+{}+\n|",
        num_combinations,
        champ_forces.len(),
        rayon::current_num_threads(),
        "-".repeat(num_chunks)
    );
    stdout().flush().unwrap();

    let mut chunks_top_n_comps = Vec::new();
    (0..num_chunks)
        .into_par_iter()
        .map(|init_index| {
            // Combination generation boiler plate
            let k_sub_1 = num_units - 1;
            let n_sub_k = num_champs - num_units;
            let mut indices: Vec<usize> = (init_index..init_index + num_units).collect();
            let mut min_heap: BinaryHeap<Reverse<CompSynergy>> = BinaryHeap::with_capacity(top_n);

            while indices[0] == init_index && indices[k_sub_1] < num_champs {
                // Calculate the amount of active synergies
                let synergy = calc_synergies(
                    &indices,
                    &champ_forces,
                    calc_unique_synergies,
                );

                // Add comp to top N
                let min = min_heap.peek().map_or(0, |m| m.0.synergy);
                if min_heap.len() == top_n {
                    if synergy > min {
                        min_heap.pop();
                        min_heap.push(Reverse(CompSynergy {
                            synergy,
                            indices: indices.clone(),
                        }));
                    }
                } else if synergy >= min {
                    min_heap.push(Reverse(CompSynergy {
                        synergy,
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
    for comp_synergy in top_n_comps {
        println!("{}", comp_synergy.0);
    }
    print!("Press enter to continue...");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn n_choose_k(n: usize, k: usize) -> usize {
    let mut combs = 1;
    for i in 0..k {
        combs = (combs * (n - i)) / (i + 1);
    }
    combs
}