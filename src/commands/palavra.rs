use rand::{random, thread_rng, Rng};
use rand::seq::SliceRandom;

use serenity::builder::CreateApplicationCommand;

const VOGAIS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const CONSOANTES: [char; 14] = ['b', 'c', 'd', 'f', 'g', 'j', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't'];
const DIGRAFOS: [char; 3] = ['s', 'c', 'ç'];

pub fn run() -> String {
    let mut rng = thread_rng();
    let mut palavra = String::new();
    let silabas: u8 = rng.gen_range(2..=5);
    for n in 1..=silabas {
        if n == 1 && random::<bool>() {
            palavra.push(*VOGAIS.choose(&mut rng).unwrap())
        }
        palavra.push(*CONSOANTES.choose(&mut rng).unwrap());
        if palavra.ends_with('q') { palavra.push('u') };
        if random::<bool>() {
            match palavra.chars().last().unwrap() {
                'c' => {palavra.push('h')},
                'g' => {palavra.push('u')},
                'l' => {palavra.push('h')},
                'n' => {palavra.push('h')},
                'r' => {palavra.push('r')},
                's' => {palavra.push(*DIGRAFOS.choose(&mut rng).unwrap())},
                _ => {}
            }
        }
        palavra.push(*VOGAIS.choose(&mut rng).unwrap());
        while palavra.ends_with("uu") {
            palavra.pop();
            palavra.push(*VOGAIS.choose(&mut rng).unwrap());
        }
    };
    palavra
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("palavra").description("faze palavra")
}
