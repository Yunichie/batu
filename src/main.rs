use rand::Rng;
use std::io;

fn main() {
    println!("Permainan Klasik - Batu");
    println!("Apakah anda ingin membaca peraturan permainan? [ya/tidak]");

    let peraturan: Vec<String> = Vec::new();

    while peraturan.is_empty() {

        let mut read_peraturan = String::new();
        io::stdin().read_line(&mut read_peraturan).unwrap();

        let read_peraturan: String = match read_peraturan.trim().parse() {
            Ok(string) => string,
            Err(_) => {
                println!("Harap masukkan input yang valid!");
                continue;
            }
        };

        if read_peraturan == "ya" {
            println!("Peraturan Permainan");
            println!("
            1. Masing-masing pemain mengambil batu sebanyak 1 - 100 (inklusif).
            2. Batu yang diambil setiap pemain akan dijumlahkan dan 
               hasil dari penjumlahan adalah jumlah batu yang akan dimainkan.
            3. Setiap pemain hanya diperbolehkan mengambil batu sebanyak 1 - 5 (inklusif) dalam setiap ronde.
            4. Setiap pemain hanya diperbolehkan bermain satu kali, lalu dilanjutkan ke pemain lawan.
            5. Pemain pertama yang berhasil menghabiskan batu dinyatakan sebagai pemenang.");
            println!("\nPermainan dimulai!");
            println!("Masukkan tingkat yang ingin dimainkan:");
            break;
        } else if read_peraturan == "tidak" {
            println!("Permainan dimulai!");
            println!("Masukkan tingkat kesulitan yang ingin dimainkan:");
            break;
        } else {
            println!("Harap masukkan input yang valid! [ya/tidak]");
            continue;
        }
    }

    let mut tingkat_kesulitan: Vec<String> = Vec::new();

    while tingkat_kesulitan.is_empty() {

        let mut read_kesulitan = String::new();
        io::stdin().read_line(&mut read_kesulitan).unwrap();

        let read_kesulitan: String = match read_kesulitan.trim().parse() {
            Ok(string) => string,
            Err(_) => {
                println!("Harap masukkan input yang valid!");
                continue;
            }
        };

        if read_kesulitan == "mudah" || read_kesulitan == "sulit" {
            tingkat_kesulitan.push(read_kesulitan);
            println!("Anda bermain dengan tingkat kesulitan: {}", tingkat_kesulitan[0]);
            println!("Masukkan batu yang ingin dimainkan:");
            break;
        } else {
            println!("Anda hanya boleh memasukkan \"mudah\" atau \"sulit\"!");
            continue;
        }
    }
    
    let batu_comp = ((rand::thread_rng().gen_range(1.0..100.0) as f32).round()) as u32;
    let mut batu_user: Vec<u32> = Vec::new();

    while batu_user.is_empty() {

        let mut read_batu_user = String::new();
        io::stdin().read_line(&mut read_batu_user).unwrap();
        
        let read_batu_user: u32 = match read_batu_user.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Harap masukkan angka yang valid!");
                continue;
            }
        };

        if read_batu_user > 100 || read_batu_user < 1 {
            println!("Kamu hanya bisa memasukkan batu sebanyak 1 <= batu <= 100 batu!");
            continue;
        } else {
            batu_user.push(read_batu_user);
            break;
        }
    }

    let mut jumlah_batu: u32 = batu_user[0] + batu_comp;

    let mut ronde = 1;

    while jumlah_batu > 0 || jumlah_batu == 0 {
        
        if jumlah_batu == 0 {
            println!("Pemenangnya adalah Computer!");
            break;
        }

        println!("\nJumlah batu yang akan dimainkan pada ronde ini: {}; ronde {}", jumlah_batu, ronde);
        println!("User dipersilahkan untuk bermain terlebih dahulu.");
        println!("Batu yang akan diambil: ");

        let mut comp_ambil_batu = ((rand::thread_rng().gen_range(1.0..5.0) as f32).round()) as u32;
        let mut user_ambil_batu = String::new();
        io::stdin().read_line(&mut user_ambil_batu).unwrap();

        let user_ambil_batu: u32 = match user_ambil_batu.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Harap masukkan angka yang valid!");
                continue;
            }
        };

        if user_ambil_batu >= 1 && user_ambil_batu <= 5 && user_ambil_batu < jumlah_batu {
            jumlah_batu -= user_ambil_batu;
            println!("Kamu mengambil {} batu! (sisa batu setelah giliranmu: {})", user_ambil_batu, jumlah_batu);
        } else if user_ambil_batu < 1 || user_ambil_batu > 5 {
            println!("Kamu hanya bisa mengambil 1 <= batu <= 5 batu!");
            continue;
        } else if jumlah_batu <= 5 && user_ambil_batu > jumlah_batu {
            println!("Kamu hanya bisa mengambil paling banyak {} batu!", jumlah_batu);
            continue;
        } else {
            println!("Kamu mengambil {} batu!", user_ambil_batu);
            jumlah_batu -= user_ambil_batu;
        }

        if jumlah_batu == 0 {
            println!("Pemenangnya adalah User!");
            break;
        } else if jumlah_batu == 1 {
            comp_ambil_batu = 1;
        } 
        
        if jumlah_batu < 5 && tingkat_kesulitan[0] == "mudah" {
            comp_ambil_batu = ((rand::thread_rng().gen_range(1.0..(jumlah_batu as f32)) as f32).round()) as u32
        } else if jumlah_batu <= 5 && tingkat_kesulitan[0] == "sulit" {
            comp_ambil_batu = jumlah_batu;
        }
        
        if jumlah_batu >= 1 {

            if tingkat_kesulitan[0] == "sulit" && jumlah_batu >= 5 {
                comp_ambil_batu = 5;
                println!("Computer mengambil {} batu!", comp_ambil_batu);
                jumlah_batu -= comp_ambil_batu;
            } else if tingkat_kesulitan[0] == "sulit" && jumlah_batu < 5 {
                println!("Computer mengambil {} batu!", comp_ambil_batu);
                jumlah_batu -= comp_ambil_batu;
            }

            if tingkat_kesulitan[0] == "mudah" {
                println!("Computer mengambil {} batu!", comp_ambil_batu);
                jumlah_batu -= comp_ambil_batu;
                println!("Sisa batu pada ronde ini: {}", jumlah_batu);
                ronde += 1;
            }
            
        }
    }
}