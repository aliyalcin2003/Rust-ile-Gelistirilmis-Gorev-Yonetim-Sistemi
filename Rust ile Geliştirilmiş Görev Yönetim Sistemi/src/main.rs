use std::io;

struct Gorev {
    id: u32,
    ad: String,
}

fn main() {
    let mut gorevler: Vec<Gorev> = Vec::new();
    let mut sonraki_id = 1;

    loop {
        println!("\n=== GÖREV YÖNETİM SİSTEMİ ===");
        println!("1 - Görev Ekle");
        println!("2 - Görevleri Listele");
        println!("3 - Çıkış");
        println!("Seçiminiz:");

        let mut secim = String::new();
        io::stdin()
            .read_line(&mut secim)
            .expect("Girdi okunamadı");

        match secim.trim() {
            "1" => {
                println!("Görev adı giriniz:");

                let mut gorev_adi = String::new();
                io::stdin()
                    .read_line(&mut gorev_adi)
                    .expect("Girdi okunamadı");

                gorevler.push(Gorev {
                    id: sonraki_id,
                    ad: gorev_adi.trim().to_string(),
                });

                println!("Görev eklendi.");
                sonraki_id += 1;
            }

            "2" => {
                println!("\nGörev Listesi:");

                if gorevler.is_empty() {
                    println!("Henüz görev bulunmuyor.");
                } else {
                    for gorev in &gorevler {
                        println!("{} - {}", gorev.id, gorev.ad);
                    }
                }
            }

            "3" => {
                println!("Program kapatılıyor...");
                break;
            }

            _ => println!("Geçersiz seçim."),
        }
    }
}