mod gioco;
use colored::Colorize;


fn lanciamo() -> u32 {
    let dadi = vec![6, 6, 6];
    let (punteggio, risultati) =  gioco::lanciatore::Lanciatore::new(dadi).lancia();
    println!("{:?}", risultati);
    return punteggio;
}

fn color_string(string: &String, red: bool) -> String {
    if red {
        return format!("{}", string).red().to_string();
    }

    return format!("{}", string).green().to_string();
}
fn main() {
    let mut pti_g1 = 0;
    let mut pti_g2 = 0;
    let mut c = 0;
    while {
        print!("Lancia Giocatore 1: ");
        pti_g1 += lanciamo();
        print!("Lancia Giocatore 2: ");
        pti_g2 += lanciamo();

        println!();
        c += 1;
        (c < 20) | (pti_g1 != pti_g2)
    } {}

    // thread::sleep(Duration::from_millis(100));

    println!("Partite Giocate: {}", c);
    println!(
        "{0: <24}: {1: <20}",
        "Punteggio Antongiacomo",
        color_string(&pti_g1.to_string(), pti_g2 >= pti_g1)
    );
    println!(
        "{0: <24}: {1: <20}",
        "Punteggio Giorgio",
        color_string(&pti_g2.to_string(), pti_g1 >= pti_g2)
    );
}
