use rayon::prelude::*;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt,
    io::{stdin, stdout, Read, Write},
    time::SystemTime,
};

// Try a non branchless approach where we skip the assignment if 3rd is empty
// Try another approach where we remove empty and opt for a pointer to a exact size array

const ABOMINATION: usize = 0;
const COVEN: usize = 1;
const DAWNBRINGER: usize = 2;
const DRACONIC: usize = 3;
const DRAGONSLAYER: usize = 4;
const ETERNAL: usize = 5;
const FORGOTTEN: usize = 6;
const HELLION: usize = 7;
const IRONCLAD: usize = 8;
const NIGHTBRINGER: usize = 9;
const REDEEMED: usize = 10;
const REVENANT: usize = 11;
const VERDANT: usize = 12;

const ASSASSIN: usize = 13;
const BRAWLER: usize = 14;
const CARETAKER: usize = 15;
const CAVALIER: usize = 16;
const CRUEL: usize = 17;
const GODKING: usize = 18;
const INVOKER: usize = 19;
const KNIGHT: usize = 20;
const LEGIONNAIRE: usize = 21;
const MYSTIC: usize = 22;
const RANGER: usize = 23;
const RENEWER: usize = 24;
const SKIRMISHER: usize = 25;
const SPELLWEAVER: usize = 26;

const EMPTY: usize = 27; // Add empty for fixed sized array and branchless implementation
const SYNERGIES: usize = EMPTY + 1;

const SYNERGY_NAMES: [&str; EMPTY] = [
    "abomination",
    "coven",
    "dawnbringer",
    "draconic",
    "dragonslayer",
    "eternal",
    "forgotten",
    "hellion",
    "ironclad",
    "nightbringer",
    "redeemed",
    "revenant",
    "verdant",
    "assassin",
    "brawler",
    "caretaker",
    "cavalier",
    "cruel",
    "godking",
    "invoker",
    "knight",
    "legionnaire",
    "mystic",
    "ranger",
    "renewer",
    "skirmisher",
    "spellweaver",
];

const NUM_CHAMPS: usize = 58;
const NUM_CHAMPS_NO_FIVE: usize = 50;
const CHAMPS: [[usize; 3]; NUM_CHAMPS] = [
    [REDEEMED, LEGIONNAIRE, EMPTY],         // Aatrox
    [NIGHTBRINGER, RANGER, EMPTY],          // Aphelios
    [VERDANT, DRACONIC, RANGER],            // Ashe
    [ABOMINATION, SPELLWEAVER, EMPTY],      // Brand
    [NIGHTBRINGER, DRAGONSLAYER, ASSASSIN], // Diana
    [FORGOTTEN, LEGIONNAIRE, EMPTY],        // Draven
    [DAWNBRINGER, BRAWLER, EMPTY],          // Gragas
    [FORGOTTEN, CAVALIER, EMPTY],           // Hecarim
    [REVENANT, INVOKER, RENEWER],           // Ivern
    [IRONCLAD, SKIRMISHER, EMPTY],          // Jax
    [ABOMINATION, LEGIONNAIRE, EMPTY],      // Kalista
    [DAWNBRINGER, INVOKER, EMPTY],          // Karma
    [FORGOTTEN, ASSASSIN, EMPTY],           // Katarina
    [HELLION, SKIRMISHER, EMPTY],           // Kennen
    [DAWNBRINGER, ASSASSIN, EMPTY],         // Khazix
    [HELLION, CAVALIER, EMPTY],             // Kled
    [COVEN, ASSASSIN, EMPTY],               // Leblanc
    [NIGHTBRINGER, SKIRMISHER, EMPTY],      // LeeSin
    [REDEEMED, KNIGHT, EMPTY],              // Leona
    [COVEN, RENEWER, EMPTY],                // Lissandra
    [HELLION, MYSTIC, EMPTY],               // Lulu
    [REDEEMED, MYSTIC, EMPTY],              // Lux
    [DRAGONSLAYER, LEGIONNAIRE, EMPTY],     // Mordekaiser
    [COVEN, NIGHTBRINGER, MYSTIC],          // Morgana
    [IRONCLAD, KNIGHT, EMPTY],              // Nautilus
    [DAWNBRINGER, SKIRMISHER, EMPTY],       // Nidalee
    [REVENANT, ASSASSIN, EMPTY],            // Nocturne
    [ABOMINATION, BRAWLER, EMPTY],          // Nunu
    [DRAGONSLAYER, SKIRMISHER, EMPTY],      // Pantheon
    [HELLION, KNIGHT, EMPTY],               // Poppy
    [REDEEMED, IRONCLAD, CAVALIER],         // Rell
    [DAWNBRINGER, LEGIONNAIRE, EMPTY],      // Riven
    [ABOMINATION, FORGOTTEN, MYSTIC],       // Ryze
    [NIGHTBRINGER, CAVALIER, EMPTY],        // Sejuani
    [DRACONIC, BRAWLER, EMPTY],             // Sett
    [DAWNBRINGER, RENEWER, EMPTY],          // Soraka
    [REDEEMED, INVOKER, EMPTY],             // Syndra
    [VERDANT, KNIGHT, EMPTY],               // Taric
    [FORGOTTEN, KNIGHT, EMPTY],             // Thresh
    [DRAGONSLAYER, SKIRMISHER, EMPTY],      // Trundle
    [DRACONIC, SKIRMISHER, EMPTY],          // Udyr
    [REDEEMED, RANGER, EMPTY],              // Varus
    [FORGOTTEN, RANGER, EMPTY],             // Vayne
    [REDEEMED, SPELLWEAVER, EMPTY],         // Velkoz
    [FORGOTTEN, SPELLWEAVER, EMPTY],        // Viktor
    [NIGHTBRINGER, RENEWER, EMPTY],         // Vladmir
    [FORGOTTEN, BRAWLER, EMPTY],            // Warwick
    [NIGHTBRINGER, LEGIONNAIRE, EMPTY],     // Yasuo
    [HELLION, SPELLWEAVER, EMPTY],          // Ziggs
    [DRACONIC, SPELLWEAVER, EMPTY],         // Zyra
    [NIGHTBRINGER, KNIGHT, GODKING],        // Darius
    [DAWNBRINGER, KNIGHT, GODKING],         // Garen
    [DRACONIC, CARETAKER, RENEWER],         // Heimerdinger
    [REDEEMED, VERDANT, LEGIONNAIRE],       // Kayle
    [ETERNAL, MYSTIC, RANGER],              // Kindred
    [HELLION, CRUEL, INVOKER],              // Teemo
    [FORGOTTEN, ASSASSIN, SKIRMISHER],      // Viego
    [REVENANT, BRAWLER, EMPTY],             // Volibear
];

const CHAMP_NAMES: [&str; NUM_CHAMPS] = [
    "aatrox",
    "aphelios",
    "ashe",
    "brand",
    "diana",
    "draven",
    "gragas",
    "hecarim",
    "ivern",
    "jax",
    "kalista",
    "karma",
    "katarina",
    "kennen",
    "khazix",
    "kled",
    "leblanc",
    "leesin",
    "leona",
    "lissandra",
    "lulu",
    "lux",
    "mordekaiser",
    "morgana",
    "nautilus",
    "nidalee",
    "nocturne",
    "nunu",
    "pantheon",
    "poppy",
    "rell",
    "riven",
    "ryze",
    "sejuani",
    "sett",
    "soraka",
    "syndra",
    "taric",
    "thresh",
    "trundle",
    "udyr",
    "varus",
    "vayne",
    "velkoz",
    "viktor",
    "vladmir",
    "warwick",
    "yasuo",
    "ziggs",
    "zyra",
    "darius",
    "garen",
    "heimerdinger",
    "kayle",
    "kindred",
    "teemo",
    "viego",
    "volibear",
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct CompSynergy {
    synergy: usize,
    indices: Vec<usize>,
}

impl fmt::Display for CompSynergy {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let champ_names: Vec<&str> = self
            .indices
            .iter()
            .map(|i| return CHAMP_NAMES[*i])
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
            .map(|champ| {
                CHAMP_NAMES
                    .iter()
                    .position(|name| *name == champ.trim().to_lowercase().as_str())
                    .expect("invalid champ name")
            })
            .collect()
    };

    print!("Force synergies (e.g. \"assassin:4, brawler:2\"): ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let trim = input_text.trim();
    let synergy_forces: Vec<[usize; 2]> = if trim.is_empty() {
        vec![]
    } else {
        trim.split(",")
            .map(|synergy_str| {
                let mut synergy_parts = synergy_str.split(":");
                // [0] is the synergy index, [1] is the minimum required
                let input_name = synergy_parts
                    .next()
                    .expect("synergy force missing name")
                    .trim()
                    .to_lowercase();

                [
                    SYNERGY_NAMES
                        .iter()
                        .position(|name| *name == input_name)
                        .expect("invalid synergy name"),
                    synergy_parts
                        .next()
                        .expect("synergy missing min number")
                        .trim()
                        .parse()
                        .expect("could not parse number"),
                ]
            })
            .collect()
    };

    print!("Number of results: [20] ");
    stdout().flush().unwrap();
    input_text.clear();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read input");
    let trim = input_text.trim();
    let top_n: usize = if trim.is_empty() {
        10
    } else {
        trim.parse().expect("could not parse number")
    };

    let time_start = SystemTime::now();

    // Log num comps and threads
    let num_combinations = n_choose_k(num_champs, num_units);
    let num_chunks = num_champs - num_units + 1;
    print!(
        "Generating and analyzing {} comps with {} champs and {} synergies forced using {} threads\n+{}+\n|",
        num_combinations,
        champ_forces.len(),
        synergy_forces.len(),
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
                    &synergy_forces,
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

fn calc_synergies(
    comp: &Vec<usize>,
    champ_forces: &Vec<usize>,
    synergy_forces: &Vec<[usize; 2]>,
    calc_unique_synergies: bool,
) -> usize {
    if !champ_forces.iter().all(|champ| comp.contains(champ)) {
        return 0;
    }

    let mut synergy_tally = [0usize; SYNERGIES];
    for champ_index in comp {
        for synergy in &CHAMPS[*champ_index] {
            synergy_tally[*synergy] += 1;
        }
    }

    if synergy_forces
        .iter()
        .any(|synergy_force| synergy_tally[synergy_force[0]] < synergy_force[1])
    {
        return 0;
    }

    let mut active_synergies = 0;
    if synergy_tally[ABOMINATION] >= 3 {
        active_synergies += synergy_tally[ABOMINATION]
    }
    if synergy_tally[COVEN] >= 3 {
        active_synergies += 3
    }
    active_synergies += synergy_tally[DAWNBRINGER] / 2 * 2;
    match synergy_tally[DRACONIC] {
        3 | 5 => active_synergies += synergy_tally[DRACONIC],
        _ => (),
    }
    active_synergies += synergy_tally[DRAGONSLAYER] / 2 * 2;
    active_synergies += synergy_tally[FORGOTTEN] / 3 * 3;
    match synergy_tally[HELLION] {
        3 | 5 | 7 => active_synergies += synergy_tally[HELLION],
        _ => (),
    }
    if synergy_tally[IRONCLAD] >= 2 {
        active_synergies += synergy_tally[IRONCLAD]
    }
    active_synergies += synergy_tally[NIGHTBRINGER] / 2 * 2;
    active_synergies += synergy_tally[REDEEMED] / 3 * 3;
    if synergy_tally[REVENANT] >= 2 {
        active_synergies += synergy_tally[REVENANT]
    }
    if synergy_tally[VERDANT] >= 2 {
        active_synergies += synergy_tally[VERDANT]
    }
    active_synergies += synergy_tally[ASSASSIN] / 2 * 2;
    active_synergies += synergy_tally[BRAWLER] / 2 * 2;
    if synergy_tally[CAVALIER] >= 2 {
        active_synergies += synergy_tally[CAVALIER]
    }
    active_synergies += synergy_tally[INVOKER] / 2 * 2;
    active_synergies += synergy_tally[KNIGHT] / 2 * 2;
    active_synergies += synergy_tally[LEGIONNAIRE] / 2 * 2;
    active_synergies += synergy_tally[MYSTIC] / 2 * 2;
    active_synergies += synergy_tally[RANGER] / 2 * 2;
    active_synergies += synergy_tally[RENEWER] / 2 * 2;
    active_synergies += synergy_tally[SKIRMISHER] / 3 * 3;
    active_synergies += synergy_tally[SPELLWEAVER] / 2 * 2;
    if calc_unique_synergies {
        active_synergies += synergy_tally[ETERNAL];
        active_synergies += synergy_tally[CARETAKER];
        active_synergies += synergy_tally[CRUEL];
        if synergy_tally[GODKING] >= 1 {
            active_synergies += 1
        }
    }
    active_synergies
}

/*
 * n Amount of total options
 * k Amount to choose from of the options
 * returns Number of possible combinations
 */
fn n_choose_k(n: usize, k: usize) -> usize {
    let mut combs = 1;
    for i in 0..k {
        combs = (combs * (n - i)) / (i + 1);
    }
    combs
}
