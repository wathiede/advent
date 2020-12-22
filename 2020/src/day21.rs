//! --- Day 21: Allergen Assessment ---
//! You reach the train's last stop and the closest you can get to your vacation island without getting wet. There aren't even any boats here, but nothing can stop you now: you build a raft. You just need a few days' worth of food for your journey.
//!
//! You don't speak the local language, so you can't read any ingredients lists. However, sometimes, allergens are listed in a language you do understand. You should be able to use this information to determine which ingredient contains which allergen and work out which foods are safe to take with you on your trip.
//!
//! You start by compiling a list of foods (your puzzle input), one food per line. Each line includes that food's ingredients list followed by some or all of the allergens the food contains.
//!
//! Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen. Allergens aren't always marked; when they're listed (as in (contains nuts, shellfish) after an ingredients list), the ingredient that contains each listed allergen will be somewhere in the corresponding ingredients list. However, even if an allergen isn't listed, the ingredient that contains that allergen could still be present: maybe they forgot to label it, or maybe it was labeled in a language you don't know.
//!
//! For example, consider the following list of foods:
//!
//! mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
//! trh fvjkl sbzzf mxmxvkd (contains dairy)
//! sqjhc fvjkl (contains soy)
//! sqjhc mxmxvkd sbzzf (contains fish)
//! The first food in the list has four ingredients (written in a language you don't understand): mxmxvkd, kfcds, sqjhc, and nhms. While the food might contain other allergens, a few allergens the food definitely contains are listed afterward: dairy and fish.
//!
//! The first step is to determine which ingredients can't possibly contain any of the allergens in any food in your list. In the above example, none of the ingredients kfcds, nhms, sbzzf, or trh can contain an allergen. Counting the number of times any of these ingredients appear in any ingredients list produces 5: they all appear once each except sbzzf, which appears twice.
//!
//! Determine which ingredients cannot possibly contain any of the allergens in your list. How many times do any of those ingredients appear?
//!
//! --- Part Two ---
//! Now that you've isolated the inert ingredients, you should have enough information to figure out which ingredient contains which allergen.
//!
//! In the above example:
//!
//! mxmxvkd contains dairy.
//! sqjhc contains fish.
//! fvjkl contains soy.
//! Arrange the ingredients alphabetically by their allergen and separate them by commas to produce your canonical dangerous ingredient list. (There should not be any spaces in your canonical dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.
//!
//! Time to stock your raft with supplies. What is your canonical dangerous ingredient list?

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = ();
    fn from_str(s: &str) -> Result<Food, ()> {
        let ingredients = s
            .split(' ')
            .take_while(|s| !s.starts_with('('))
            .map(|s| s.to_string())
            .collect();
        let allergens = s
            .split(' ')
            .skip_while(|s| !s.starts_with('('))
            .skip(1)
            .map(|s| s.trim_matches(&[',', ')'][..]).to_string())
            .collect();
        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

impl Food {}
fn count_ingredients(foods: &[Food], ingredients: &HashSet<String>) -> usize {
    foods
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|i| ingredients.contains(*i))
                .count()
        })
        .sum()
}

#[aoc_generator(day21)]
fn generator(input: &str) -> Vec<Food> {
    input
        .split('\n')
        .map(|s| s.parse().expect("couldn't parse food"))
        .collect()
}

fn find_non_allergens(foods: &[Food]) -> HashSet<String> {
    // Find ingredients common across all foods for a given allergen.  The remaining ingredients
    // are non-allergens.
    //
    let mut allergen_map = HashMap::new();
    let mut ingredient_map = HashMap::new();
    foods.iter().for_each(|f| {
        f.allergens.iter().for_each(|allergen| {
            let a = allergen_map.entry(allergen).or_insert(0);
            *a += 1;
            f.ingredients.iter().for_each(|ingredient| {
                let i = ingredient_map
                    .entry(ingredient)
                    .or_insert(HashMap::new())
                    .entry(allergen)
                    .or_insert(0);
                *i += 1;
            });
        });
    });

    //  "nhms": {
    //     "dairy": 1,
    //     "fish": 1,
    // },
    // "fvjkl": {
    //     "soy": 1,
    //     "dairy": 1,
    // },

    ingredient_map
        .iter()
        .filter(|(_, v)| !v.iter().any(|(a, c)| &allergen_map[a] == c))
        .map(|(k, _)| k.to_string())
        .collect()

    /*
    dbg!(&allergen_map);
    dbg!(&ingredient_map);
    let allergens: HashSet<String> = foods
    .iter()
    .map(|f| f.allergens.iter())
    .flatten()
    .cloned()
    .collect();
    HashSet::new()
    */
}

#[aoc(day21, part1)]
fn solution1(foods: &[Food]) -> usize {
    let ingredients = find_non_allergens(foods);
    count_ingredients(&foods, &ingredients)
}

fn allergen_ingredients(foods: &[Food], non_allergens: &HashSet<String>) -> Vec<(String, String)> {
    let mut allergen_only = HashMap::new();
    foods.iter().for_each(|food| {
        print!("{:?}:", food.allergens);
        food.ingredients.iter().for_each(|i| {
            for a in &food.allergens {
                let v = allergen_only
                    .entry(a)
                    .or_insert(HashMap::new())
                    .entry(i)
                    .or_insert(0);
                *v += 1;
            }
            if !non_allergens.contains(i) {
                print!(" {}", i);
            }
        });
        food.ingredients.iter().for_each(|i| {
            if non_allergens.contains(i) {
                print!(" *{}", i);
            }
        });
        println!();
    });
    // TODO walk over allergen_only.  Find the entries with a single max value, and that's the
    // allergen/ingredient combo.  Then remove that ingredient from all the other entries, and
    // repeat until nothing is left.
    dbg!(&allergen_only);
    let mut answer = Vec::new();
    let mut limit = 0;
    loop {
        dbg!(&allergen_only);
        if allergen_only.is_empty() {
            return answer;
        };
        let mut rm = ("".to_string(), "".to_string());
        allergen_only.iter().for_each(|(a, i_counts)| {
            let max = i_counts.values().max().unwrap();
            if i_counts.iter().filter(|(_i, c)| c == &max).count() == 1 {
                let i = i_counts
                    .iter()
                    .filter(|(_i, c)| c == &max)
                    .map(|(i, _c)| i)
                    .nth(0)
                    .unwrap();
                answer.push((a.to_string(), i.to_string()));
                rm = (a.to_string(), i.to_string());
            }
        });
        println!("removing {:?}", rm);
        allergen_only.iter_mut().for_each(|(_, i_counts)| {
            i_counts.remove(&rm.1);
        });
        allergen_only.remove(&rm.0);
        limit += 1;
        if limit > 10 {
            panic!()
        };
    }
}

#[aoc(day21, part2)]
fn solution2(foods: &[Food]) -> String {
    let non_allergens = find_non_allergens(foods);
    let mut allergens = allergen_ingredients(foods, &non_allergens);
    allergens.sort_by(|l, r| l.0.cmp(&r.0));
    dbg!(&allergens);
    allergens
        .iter()
        .map(|(_, a)| a.as_str())
        .collect::<Vec<_>>()
        .as_slice()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

    #[test]
    fn parse() {
        let foods = generator(INPUT);
        assert_eq!(foods.len(), 4);
        assert_eq!(
            foods[0].ingredients,
            ["mxmxvkd", "kfcds", "sqjhc", "nhms"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            foods[0].allergens,
            ["dairy", "fish"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn part1() {
        assert_eq!(solution1(&generator(INPUT)), 5);
    }
    #[test]
    fn part2() {
        assert_eq!(solution2(&generator(INPUT)), "mxmxvkd,sqjhc,fvjkl");
    }
    #[test]
    fn non_allergens() {
        assert_eq!(
            find_non_allergens(&generator(INPUT)),
            ["kfcds", "nhms", "sbzzf", "trh"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    #[test]
    fn count() {
        let ingredients: HashSet<String> = vec!["kfcds", "nhms", "sbzzf", "trh"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let foods = generator(INPUT);
        assert_eq!(count_ingredients(&foods, &ingredients), 5);
    }
}
