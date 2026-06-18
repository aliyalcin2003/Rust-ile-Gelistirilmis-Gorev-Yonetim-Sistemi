use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use colored::Colorize;
use chrono::Local; // Tarih ve saat kütüphanesi dahil edildi

#[derive(Serialize, Deserialize)]
struct Gorev {
    id: u32,
    kullanici: String,
    ad: String,
    tamamlandi: bool,
    tarih: String, // Görevlere tarih özelliği eklendi
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
    println!("{}", "\n===== GÜVENLİK KONTROLÜ =====".cyan().bold());
    println!("Sisteme giriş yapmak için lütfen şifreyi giriniz:");
    
    let mut sifre = String::new();
    io::stdin().read_line(&mut sifre).unwrap();

    if sifre.trim() != "admin123" {
        println!("{}", "Hatalı şifre! Sisteme erişim reddedildi.".red().bold());
        return; 
    }
    
    println!("{}", "Giriş başarılı! Sistem başlatılıyor...\n".green().bold());

    let mut gorevler = gorevleri_yukle();
    let mut sonraki_id = gorevler.iter().map(|g| g.id).max().unwrap_or(0) + 1;

    loop {
        println!("{}", "\n===== GÖREV YÖNETİM SİSTEMİ =====".cyan().bold());
        println!("1 - Görev Ekle");
        println!("2 - Görevleri Listele");
        println!("3 - Görev Sil");
        println!("4 - Tüm Görevleri Sil");
        println!("5 - Görevi Tamamla");
        println!("6 - Görev Güncelle");
        println!("7 - İstatistik Göster");
        println!("8 - Tamamlanan Görevleri Temizle");
        println!("9 - Kullanıcı Seçenekleri (Listele / Sil)");
        println!("10 - Simülasyon Başlat (Test)");
        println!("{}", "11 - Çıkış".red());
        println!("Seçiminizi giriniz:");

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).unwrap();

        match secim.trim() {
            "1" => {
                println!("Kullanıcı adı:");
                let mut kullanici = String::new();
                io::stdin().read_line(&mut kullanici).unwrap();

                println!("Görev adı giriniz:");
                let mut ad = String::new();
                io::stdin().read_line(&mut ad).unwrap();

                if ad.trim().is_empty() {
                    println!("{}", "Hata: Lütfen geçerli bir görev giriniz!".red().bold());
                    continue;
                }

                // O anki tarih ve saati gün/ay/yıl - saat/dakika formatında alıyoruz
                let anlik_tarih = Local::now().format("%d.%m.%Y - %H:%M").to_string();

                gorevler.push(Gorev {
                    id: sonraki_id,
                    kullanici: kullanici.trim().to_string(),
                    ad: ad.trim().to_string(),
                    tamamlandi: false,
                    tarih: anlik_tarih, // Alınan tarih göreve kaydediliyor
                });

                sonraki_id += 1;
                gorevleri_kaydet(&gorevler);
                println!("{}", "Görev başarıyla eklendi.".green());
            }

            "2" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Sistemde kayıtlı herhangi bir görev bulunamadı.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
                    println!("{}", "\nGörev Listesi:".cyan());
                    for gorev in &gorevler {
                        let durum = if gorev.tamamlandi { 
                            "Tamamlandı".green() 
                        } else { 
                            "Devam Ediyor".yellow() 
                        };
                        // Listelemede tarih gösterimi eflatun renkle eklendi
                        println!("{} - {} - {} [{}] (Tarih: {})", gorev.id, gorev.kullanici, gorev.ad, durum, gorev.tarih.magenta());
                    }
                    println!("\nToplam {} görev listelendi. Menüye dönmek için 'Enter' tuşuna basın...", gorevler.len());
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                }
            }

            "3" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Sistemde silinecek herhangi bir görev bulunmuyor.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
                    println!("Silmek istediğiniz görev id:");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();

                    let id: u32 = id.trim().parse().unwrap_or(0);
                    let onceki_sayi = gorevler.len();

                    gorevler.retain(|g| g.id != id);

                    if gorevler.len() < onceki_sayi {
                        gorevleri_kaydet(&gorevler);
                        println!("{}", "Görev silindi.".green());
                    } else {
                        println!("{}", "Görev bulunamadı.".red());
                    }
                }
            }

            "4" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Sistemde silinecek görev yok.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
                    println!("{}", "DİKKAT: Tüm görevleri silmek istediğinize emin misiniz? (e/h):".red().bold());
                    let mut onay = String::new();
                    io::stdin().read_line(&mut onay).unwrap();
                    
                    if onay.trim().to_lowercase() == "e" {
                        gorevler.clear();
                        sonraki_id = 1;
                        gorevleri_kaydet(&gorevler);
                        println!("{}", "Sistemdeki bütün görevler başarıyla silindi.".green().bold());
                    } else {
                        println!("{}", "İşlem iptal edildi.".yellow());
                    }
                }
            }

            "5" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Sistemde tamamlanacak herhangi bir görev bulunmuyor.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
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
                        println!("{}", "Görev tamamlandı olarak işaretlendi.".green());
                    } else {
                        println!("{}", "Görev bulunamadı.".red());
                    }
                }
            }

            "6" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Sistemde güncellenecek herhangi bir görev bulunmuyor.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
                    println!("Güncellenecek görev id:");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();

                    let id: u32 = match id.trim().parse() {
                        Ok(sayi) => sayi,
                        Err(_) => {
                            println!("{}", "Geçersiz id girdiniz.".red());
                            continue;
                        }
                    };

                    println!("Yeni görev adı:");
                    let mut yeni_ad = String::new();
                    io::stdin().read_line(&mut yeni_ad).unwrap();

                    let mut bulundu = false;

                    for gorev in &mut gorevler {
                        if gorev.id == id {
                            if gorev.ad == yeni_ad.trim() {
                                println!("{}", "Yeni görev adı mevcut görev adıyla aynı olamaz.".yellow());
                                bulundu = true;
                                break;
                            }
                            gorev.ad = yeni_ad.trim().to_string();
                            bulundu = true;
                        }
                    }
                    
                    if bulundu {
                        gorevleri_kaydet(&gorevler);
                        println!("{}", "Görev güncellendi.".green());
                    } else {
                        println!("{}", "Görev bulunamadı.".red());
                    }
                }
            }
                        
            "7" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Kayıtlı görev olmadığı için istatistik gösterilemiyor.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
                    let toplam = gorevler.len();
                    let tamamlanan = gorevler.iter().filter(|g| g.tamamlandi).count();
                    let devam_eden = toplam - tamamlanan;

                    println!("{}", "\n===== İSTATİSTİKLER =====".magenta().bold());
                    println!("Toplam Görev     : {}", toplam);
                    println!("Tamamlanan Görev : {}", tamamlanan.to_string().green());
                    println!("Devam Eden Görev : {}", devam_eden.to_string().yellow());
                    
                    println!("\nAna menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                }
            }

            "8" => {
                if gorevler.is_empty() {
                    println!("{}", "\n[!] Sistemde temizlenecek herhangi bir görev bulunmuyor.".yellow().bold());
                    println!("Ana menüye dönmek için 'Enter' tuşuna basın...");
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                } else {
                    let onceki_sayi = gorevler.len();
                    gorevler.retain(|g| !g.tamamlandi);
                    let silinen = onceki_sayi - gorevler.len();

                    gorevleri_kaydet(&gorevler);
                    println!("{} {}", silinen, "adet tamamlanmış görev silindi.".green());
                }
            }

            "9" => {
                loop {
                    println!("{}", "\n===== KULLANICI SEÇENEKLERİ =====".cyan().bold());
                    println!("1 - Kullanıcıları Listele");
                    println!("2 - Kullanıcı Sil");
                    println!("3 - Ana Menüye Dön");
                    println!("Seçiminizi giriniz:");

                    let mut k_secim = String::new();
                    io::stdin().read_line(&mut k_secim).unwrap();

                    match k_secim.trim() {
                        "1" => {
                            if gorevler.is_empty() {
                                println!("{}", "\n[!] Sistemde kayıtlı kullanıcı bulunmadığı için işlem yapılamıyor.".yellow().bold());
                                println!("Kullanıcı menüsüne dönmek için 'Enter' tuşuna basın...");
                                let mut bekle = String::new();
                                io::stdin().read_line(&mut bekle).unwrap();
                            } else {
                                println!("{}", "\nSistemdeki Kullanıcılar:".cyan());
                                let mut benzersiz_kullanicilar: Vec<String> = Vec::new();
                                for gorev in &gorevler {
                                    if !benzersiz_kullanicilar.contains(&gorev.kullanici) {
                                        benzersiz_kullanicilar.push(gorev.kullanici.clone());
                                    }
                                }
                                for (index, kullanici) in benzersiz_kullanicilar.iter().enumerate() {
                                    println!("{}. {}", index + 1, kullanici.blue());
                                }
                                println!("\nMenüye dönmek için 'Enter' tuşuna basın...");
                                let mut bekle = String::new();
                                io::stdin().read_line(&mut bekle).unwrap();
                            }
                        }
                        "2" => {
                            if gorevler.is_empty() {
                                println!("{}", "\n[!] Sistemde kayıtlı kullanıcı bulunamadığı için işlem yapılamıyor.".yellow().bold());
                                println!("Kullanıcı menüsüne dönmek için 'Enter' tuşuna basın...");
                                let mut bekle = String::new();
                                io::stdin().read_line(&mut bekle).unwrap();
                            } else {
                                println!("Silmek istediğiniz kullanıcı adını tam olarak yazın:");
                                let mut silinecek_k_adi = String::new();
                                io::stdin().read_line(&mut silinecek_k_adi).unwrap();
                                let silinecek_k_adi = silinecek_k_adi.trim();

                                let onceki_sayi = gorevler.len();
                                gorevler.retain(|g| g.kullanici != silinecek_k_adi);
                                let silinen_gorev_sayisi = onceki_sayi - gorevler.len();

                                if silinen_gorev_sayisi > 0 {
                                    gorevleri_kaydet(&gorevler);
                                    let mesaj = format!("'{}' isimli kullanıcı ve ona ait {} adet görev sistemden başarıyla silindi.", silinecek_k_adi, silinen_gorev_sayisi);
                                    println!("{}", mesaj.green());
                                } else {
                                    println!("{}", "Hata: Bu isimde aktif bir kullanıcı veya görev bulunamadı.".red());
                                }
                            }
                        }
                        "3" => break,
                        _ => println!("{}", "Geçersiz seçim yaptınız.".red()),
                    }
                }
            }

            "10" => {
                println!("{}", "Simülasyon başlatılıyor, lütfen bekleyin...".yellow());
                let anlik_tarih = Local::now().format("%d.%m.%Y - %H:%M").to_string();
                
                for i in 1..=500 {
                    let simule_kullanici = format!("Kullanici_{}", (i % 1000) + 1); 
                    
                    gorevler.push(Gorev {
                        id: sonraki_id,
                        kullanici: simule_kullanici,
                        ad: format!("Sistem Test Görevi #{}", i),
                        tamamlandi: i % 3 == 0,
                        tarih: anlik_tarih.clone(), // Test görevlerine de aynı tarih basılıyor
                    });
                    
                    sonraki_id += 1;
                }
                gorevleri_kaydet(&gorevler);
                println!("{}", "Simülasyon başarıyla tamamlandı!".green().bold());
                println!("Sisteme 500 yeni görev ve rastgele kullanıcılar yüklendi.");
            }

            "11" => {
                println!("{}", "Program kapatılıyor...".yellow());
                break;
            }

            _ => {
                println!("{}", "Geçersiz seçim yaptınız.".red());
            }
        }
    }
}