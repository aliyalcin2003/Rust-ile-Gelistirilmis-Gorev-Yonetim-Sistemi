use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct Gorev {
    id: u32,
    kullanici: String,
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
    println!("===== GÜVENLİK KONTROLÜ =====");
    println!("Sisteme giriş yapmak için lütfen şifreyi giriniz:");
    
    let mut sifre = String::new();
    io::stdin().read_line(&mut sifre).unwrap();

    if sifre.trim() != "admin123" {
        println!("Hatalı şifre! Sisteme erişim reddedildi.");
        return; 
    }
    
    println!("Giriş başarılı! Sistem başlatılıyor...\n");

    let mut gorevler = gorevleri_yukle();
    
    // Güvenli ID Yönetimi: Silme işlemlerinden sonra ID'lerin çakışmasını önler
    let mut sonraki_id = gorevler.iter().map(|g| g.id).max().unwrap_or(0) + 1;

    loop {
        println!("\n===== GÖREV YÖNETİM SİSTEMİ =====");
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
        println!("11 - Çıkış");
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
                    println!("Hata: Lütfen geçerli bir görev giriniz!");
                    continue;
                }

                gorevler.push(Gorev {
                    id: sonraki_id,
                    kullanici: kullanici.trim().to_string(),
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
                        let durum = if gorev.tamamlandi { "Tamamlandı" } else { "Devam Ediyor" };
                        println!("{} - {} - {} [{}]", gorev.id, gorev.kullanici, gorev.ad, durum);
                    }
                    println!("\nToplam {} görev listelendi. Menüye dönmek için 'Enter' tuşuna basın...", gorevler.len());
                    let mut bekle = String::new();
                    io::stdin().read_line(&mut bekle).unwrap();
                }
            }

            "3" => {
                println!("Silmek istediğiniz görev id:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap_or(0);
                let onceki_sayi = gorevler.len();

                gorevler.retain(|g| g.id != id);

                if gorevler.len() < onceki_sayi {
                    gorevleri_kaydet(&gorevler);
                    println!("Görev silindi.");
                } else {
                    println!("Görev bulunamadı.");
                }
            }

            "4" => {
                if gorevler.is_empty() {
                    println!("Sistemde silinecek görev yok.");
                } else {
                    println!("DİKKAT: Tüm görevleri silmek istediğinize emin misiniz? (e/h):");
                    let mut onay = String::new();
                    io::stdin().read_line(&mut onay).unwrap();
                    
                    if onay.trim().to_lowercase() == "e" {
                        gorevler.clear();
                        sonraki_id = 1; // ID sayacını sıfırla
                        gorevleri_kaydet(&gorevler);
                        println!("Sistemdeki bütün görevler başarıyla silindi.");
                    } else {
                        println!("İşlem iptal edildi.");
                    }
                }
            }

            "5" => {
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

            "6" => {
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
                        if gorev.ad == yeni_ad.trim() {
                            println!("Yeni görev adı mevcut görev adıyla aynı olamaz.");
                            bulundu = true;
                            break;
                        }
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
                        
            "7" => {
                let toplam = gorevler.len();
                let tamamlanan = gorevler.iter().filter(|g| g.tamamlandi).count();
                let devam_eden = toplam - tamamlanan;

                println!("\n===== İSTATİSTİKLER =====");
                println!("Toplam Görev     : {}", toplam);
                println!("Tamamlanan Görev : {}", tamamlanan);
                println!("Devam Eden Görev : {}", devam_eden);
            }

            "8" => {
                let onceki_sayi = gorevler.len();
                gorevler.retain(|g| !g.tamamlandi);
                let silinen = onceki_sayi - gorevler.len();

                gorevleri_kaydet(&gorevler);
                println!("{} adet tamamlanmış görev silindi.", silinen);
            }

            "9" => {
                loop {
                    println!("\n===== KULLANICI SEÇENEKLERİ =====");
                    println!("1 - Kullanıcıları Listele");
                    println!("2 - Kullanıcı Sil");
                    println!("3 - Ana Menüye Dön");
                    println!("Seçiminizi giriniz:");

                    let mut k_secim = String::new();
                    io::stdin().read_line(&mut k_secim).unwrap();

                    match k_secim.trim() {
                        "1" => {
                            if gorevler.is_empty() {
                                println!("Sistemde henüz kayıtlı kullanıcı bulunmuyor.");
                            } else {
                                println!("\nSistemdeki Kullanıcılar:");
                                let mut benzersiz_kullanicilar: Vec<String> = Vec::new();
                                for gorev in &gorevler {
                                    if !benzersiz_kullanicilar.contains(&gorev.kullanici) {
                                        benzersiz_kullanicilar.push(gorev.kullanici.clone());
                                    }
                                }
                                for (index, kullanici) in benzersiz_kullanicilar.iter().enumerate() {
                                    println!("{}. {}", index + 1, kullanici);
                                }
                                println!("\nMenüye dönmek için 'Enter' tuşuna basın...");
                                let mut bekle = String::new();
                                io::stdin().read_line(&mut bekle).unwrap();
                            }
                        }
                        "2" => {
                            println!("Silmek istediğiniz kullanıcı adını tam olarak yazın:");
                            let mut silinecek_k_adi = String::new();
                            io::stdin().read_line(&mut silinecek_k_adi).unwrap();
                            let silinecek_k_adi = silinecek_k_adi.trim();

                            let onceki_sayi = gorevler.len();
                            // O kullanıcıya ait bütün görevleri sistemden temizle
                            gorevler.retain(|g| g.kullanici != silinecek_k_adi);
                            let silinen_gorev_sayisi = onceki_sayi - gorevler.len();

                            if silinen_gorev_sayisi > 0 {
                                gorevleri_kaydet(&gorevler);
                                println!("'{}' isimli kullanıcı ve ona ait {} adet görev sistemden başarıyla silindi.", silinecek_k_adi, silinen_gorev_sayisi);
                            } else {
                                println!("Hata: Bu isimde aktif bir kullanıcı veya görev bulunamadı.");
                            }
                        }
                        "3" => break, // Döngüden çıkıp ana menüye döner
                        _ => println!("Geçersiz seçim yaptınız."),
                    }
                }
            }

            "10" => {
                println!("Simülasyon başlatılıyor, lütfen bekleyin...");
                for i in 1..=500 {
                    let simule_kullanici = format!("Kullanici_{}", (i % 1000) + 1); 
                    
                    gorevler.push(Gorev {
                        id: sonraki_id,
                        kullanici: simule_kullanici,
                        ad: format!("Sistem Test Görevi #{}", i),
                        tamamlandi: i % 3 == 0,
                    });
                    
                    sonraki_id += 1;
                }
                gorevleri_kaydet(&gorevler);
                println!("Simülasyon başarıyla tamamlandı!");
                println!("Sisteme 500 yeni görev ve rastgele kullanıcılar yüklendi.");
            }

            "11" => {
                println!("Program kapatılıyor...");
                break;
            }

            _ => {
                println!("Geçersiz seçim yaptınız.");
            }
        }
    }
}