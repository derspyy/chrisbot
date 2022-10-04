use rand::random;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    gen_palavra().to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("palavra").description("faze palavra")
}


fn gen_palavra() -> String {

    let x = random::<u8>() % 5;
    let sil1 = x + 2;
    let mut palavra = "".to_string();
    let mut sil2 : u8 = 0;
    while sil1 > sil2 {

        if (sil2 == 0) && (random::<f32>() > 0.5)  { // chance da primeira letra ser consoante
            let z = gen_cons(sil2);
            palavra.push_str(z.as_str());
        }
        else if sil2 > 0 {
            let z = gen_cons(sil2);
            palavra.push_str(z.as_str());
        }
        let z = gen_vogs();
        palavra.push_str(z);

        sil2 += 1;
    }
    return palavra;
}

fn gen_cons(sil2: u8) -> String {
    let x = random::<u16>() % 13; // 14 - 1
    let y = x as u8;

    let letra = match y {
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "f",
        5 => "g",
        6 => "j",
        7 => "l",
        8 => "m",
        9 => "n",
        10 => "p",
        11 => "qu",
        12 => "r",
        13 => "s",
        0 => "t",
        _ => "BUGOU ARRUMA"
    };
    let mut digrafo = match letra {
        "c" => "h",
        "s" => "1",
        "r" => "r",
        "l" => "h",
        "n" => "h",
        "g" => "u",
        _ => ""
    };
    if digrafo == "1" {
        let x = random::<u16>() % 2; // 3 - 1
        let y = x as u8;
        digrafo = match y {
            1 => "s",
            2 => "c",
            0 => "ç",
            _ => ""
        }

    }
    let consoante : String;
    if (random::<f32>() > 0.5) && (sil2 > 0) {
        consoante = letra.to_string() + digrafo;
        return consoante;
    }
    else {
        return letra.to_string();
    }

}

fn gen_vogs() -> &'static str {
    let x = random::<u16>() % 4; // 5 - 1
    let y = x as u8;
    let letra = match y {
        1 => "a",
        2 => "e",
        3 => "i",
        4 => "o",
        0 => "u",
        _ => "BUGOU ARRUMA"
    };
    return letra
}