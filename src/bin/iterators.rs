#![allow(unused)]


// map simple masterclass right here sir
// fn main() {
//     let v1 = vec![1,2,3];
//     let v2 = v1.iter().map(|x| x + 2).collect::<Vec<i32>>();

//     println!("v2 {v2:?}");


//     let a: Option<i32> = Some(10);

//     let b = a.map(|x| x + 20);

//     println!("a: {a:?}");
//     println!("b: {b:?}")
// }


// anything that implements the iterator trait that is an Iterator
// trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

#[derive(Debug)]
struct Transaction {
    sender: String,
    amount: u64,
    success: bool,
}

fn main() {
    //  let nums = vec![1, 2, 3];

    // let mut iter = nums.iter();



    // println!("{:?}", iter.next()); // Some(1)
    // println!("{:?}", iter.next()); // Some(2)
    // println!("{:?}", iter.next()); // Some(3)
    // println!("{:?}", iter.next()); // None

    // this is the iterators masterclass
    // let nums  = vec![2,3,4];
    // for n in nums.iter() {
    //     // let new = n + 1; //@audit we coreced it here 
    //     println!("{n}");
    // }

    // println!("{:?}", nums);


    // let mut nums = vec![2,3,4];
    // for n in nums.iter_mut() {
    //     *n += 10;
    // }

    // println!("{:?}", nums);

    // //@note the one here is the iteration one here so we need to ensure we mapped it out right well
    // let nums = vec![1,2,3];
    // for n in nums.into_iter() {
    //     println!("{n}")
    // }

    // println!("{nums:?}");


    // let nums = vec![1, 2, 3, 4];


    // // map adapters right here 
    // let doubled = nums.iter().map(|x| x * 2);
    // println!("{:?}", doubled);


    // let new: Vec<i32> = doubled.collect();
    // println!("new guys: {new:?}");


    //filter adapters right here 
    // let nums = vec![1,2,3,4];
    // let even: Vec<&i32> = nums.iter().filter(|x| **x % 2 == 0).collect();
    // println!("{:?}", even);

    // //we have the find adapter too right here

    // let nums = vec![3,7,10,12];
    // let found = nums.iter().find(|x| **x > 8);

    // println!("{:?}", found);


    let txs = vec![
        Transaction{sender: String::from("daniel"), amount: 78, success: true},
        Transaction{sender: String::from("victor"), amount: 200, success: false},
        Transaction{sender: String::from("vincent"), amount: 7845, success: true},
    ];

    let successful_transactions: Vec<&Transaction> = txs.iter().filter(|x| (**x).success).collect();
    println!("successful txs: {:?}", successful_transactions);

    let successful_amount: Vec<u64> = txs.iter().filter(|x| ((**x).success)).map(|x| x.amount).collect();
    println!("successful txs amounts: {:?}", successful_amount);


    let total_value_sent: u64 = txs.iter().filter(|x| ((**x).success)).map(|x| x.amount).sum();
    println!("total sent: {total_value_sent}");


    let senders: Vec<&String> = txs.iter().filter(|x| x.success).map(|x| &x.sender).collect();
    println!("senders: {senders:?}")
}




