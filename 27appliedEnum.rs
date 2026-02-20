use std::io;

enum FileSize{
    Bytes (u64),
    Kilobytes (u64),
    Megabytes (u64),
    Gigabytes (u64),
}
impl FileSize{
    fn format_size(&self)-> String{
        match self{
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB",*kb as f64/1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB",*mb as f64/1_000_000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB",*gb as f64/1_000_000_000.0),
        }
    }   
}

fn main(){
    println!("Enter the size in bytes: ");
    let mut size = String::new();
    io::stdin().read_line(&mut size).unwrap();
    let size = size.trim().parse::<u64>().expect("Enter a valid number");
    let file_size = match size{
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size),
        1_000_000..=999_999_999 => FileSize::Megabytes(size),
        _=> FileSize::Gigabytes(size),
    };
    println!("The file size: {}",file_size.format_size());
}

