use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct Gorev {
    id: u32,
    ad: String,
    tamamlandi: bool,
}

fn gorevleri_kaydet(gorevler: &Vec<Gorev>) {
    let veri = serde_json::to_string_pretty(gorevler).unwrap();
    fs::write("gorevler.json", veri).unwrap();
}

fn gorevleri_yukle() -> Vec<Gorev> {
    match fs::read_to_string("gorevler.json") {
        Ok(veri) => serde_json::from_str(&veri).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn main() {
    let mut gorevler = gorevleri_yukle();

    let mut sonraki_id = gorevler.len() as u32 + 1;

    loop {
        println!("\n=== GÖREV YÖNETİM SİSTEMİ ===");
        println!("1 - Görev Ekle");
        println!("2 - Görevleri Listele");
        println!("3 - Görev Sil");
        println!("4 - Görevi Tamamla");
        println!("5 - Çıkış");

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).unwrap();

        match secim.trim() {
            "1" => {
                println!("Görev adı:");

                let mut ad = String::new();
                io::stdin().read_line(&mut ad).unwrap();

                gorevler.push(Gorev {
                    id: sonraki_id,
                    ad: ad.trim().to_string(),
                    tamamlandi: false,
                });

                sonraki_id += 1;
                gorevleri_kaydet(&gorevler);

                println!("Görev eklendi.");
            }

            "2" => {
                if gorevler.is_empty() {
                    println!("Görev bulunamadı.");
                } else {
                    for g in &gorevler {
                        let durum = if g.tamamlandi {
                            "Tamamlandı"
                        } else {
                            "Devam Ediyor"
                        };

                        println!("{} - {} [{}]", g.id, g.ad, durum);
                    }
                }
            }

            "3" => {
                println!("Silinecek görev id:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap_or(0);

                gorevler.retain(|g| g.id != id);

                gorevleri_kaydet(&gorevler);

                println!("Görev silindi.");
            }

            "4" => {
                println!("Tamamlanacak görev id:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap_or(0);

                for g in &mut gorevler {
                    if g.id == id {
                        g.tamamlandi = true;
                    }
                }

                gorevleri_kaydet(&gorevler);

                println!("Görev tamamlandı olarak işaretlendi.");
            }

            "5" => {
                println!("Program kapatılıyor...");
                break;
            }

            _ => println!("Geçersiz seçim."),
        }
    }
}