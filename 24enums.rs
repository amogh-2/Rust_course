#[derive(Debug)]

enum DiskType{
    SSD,
    _HDD,
}
#[derive(Debug)]
enum DiskSize{
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main(){
    let disktype = DiskType::SSD;
    match disktype{
        // gotcha prepare a case for each of the possible cases of type DiskSize
        DiskType::SSD => println!("The disk is a SSD"),
        DiskType::_HDD => println!("The disk is a HDD"),
        // _=> println!("Something went wrong"), Donot need this as it is unreachable there are only 2 possible cases
    }
    let disksize = DiskSize::MB(132);
    println!("The {:?} disk size is {:?}",disktype,disksize);
}