mod gioco;

fn lanciamo() -> u32{
    let dadi = vec![6, 6, 6] ;
    let (punteggio, _risultati) = gioco::lanciatore::Lanciatore::new(dadi).lancia();
    return punteggio
}

fn main() {
    let mut pti_g1 = 0;
    let mut pti_g2 = 0;

    for _i in 1..=10000{
        pti_g1 += lanciamo();
        pti_g2 += lanciamo();
        // thread::sleep(Duration::from_millis(100));
    }

    println!("Punteggio Antongiacomo: {}", pti_g1);
    println!("Punteggio Giorgio: {}", pti_g2);
}
