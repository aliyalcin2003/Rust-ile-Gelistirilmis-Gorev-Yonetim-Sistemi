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
        println!("5 - Görev Güncelle");
        println!("6 - İstatistik Göster");
        println!("7 - Çıkış");
        println!("Seçiminizi giriniz:");

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

                println!("Görev başarıyla eklendi.");
            }

            "2" => {
                if gorevler.is_empty() {
                    println!("Kayıtlı görev bulunamadı.");
                } else {
                    println!("\nGörev Listesi:");

                    for gorev in &gorevler {
                        let durum = if gorev.tamamlandi {
                            "Tamamlandı"
                        } else {
                            "Devam Ediyor"
                        };

                        println!(
                            "{} - {} [{}]",
                            gorev.id,
                            gorev.ad,
                            durum
                        );
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

                let mut bulundu = false;

                for gorev in &mut gorevler {
                    if gorev.id == id {
                        gorev.tamamlandi = true;
                        bulundu = true;
                    }
                }

                if bulundu {
                    gorevleri_kaydet(&gorevler);
                    println!("Görev tamamlandı olarak işaretlendi.");
                } else {
                    println!("Görev bulunamadı.");
                }
            }

            "5" => {
                println!("Güncellenecek görev id:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = match id.trim().parse() {
                    Ok(sayi) => sayi,
                    Err(_) => {
                        println!("Geçersiz id girdiniz.");
                        continue;
                    }
                };

                println!("Yeni görev adı:");

                let mut yeni_ad = String::new();
                io::stdin().read_line(&mut yeni_ad).unwrap();

                let mut bulundu = false;

                for gorev in &mut gorevler {
                    if gorev.id == id {
                        gorev.ad = yeni_ad.trim().to_string();
                        bulundu = true;
                    }
                }

                if bulundu {
                    gorevleri_kaydet(&gorevler);
                    println!("Görev güncellendi.");
                } else {
                    println!("Görev bulunamadı.");
                }
            }

            "6" => {
                let toplam = gorevler.len();

                let tamamlanan =
                    gorevler.iter()
                    .filter(|g| g.tamamlandi)
                    .count();

                let devam_eden = toplam - tamamlanan;

                println!("\n=== İSTATİSTİKLER ===");
                println!("Toplam Görev: {}", toplam);
                println!("Tamamlanan Görev: {}", tamamlanan);
                println!("Devam Eden Görev: {}", devam_eden);
            }

            "7" => {
                println!("Program kapatılıyor...");
                break;
            }

            _ => {
                println!("Geçersiz seçim yaptınız.");
            }
        }
    }
}