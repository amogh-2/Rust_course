fn main(){
    let mut v= vec![1,2,3];
    v.push(4);

    let more_numbers = vec![5,6,7];
    v.extend(more_numbers);

    let mut other_numbers = vec![8,9];
    v.append(&mut other_numbers);

    v.insert(0,0);
    println!("{:?}",v);
}