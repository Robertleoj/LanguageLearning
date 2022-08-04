struct Solution;


use std::collections::{BTreeSet,btree_set::IntoIter, HashMap};
use std::vec;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut name_hm: HashMap<String, Vec<BTreeSet<String>>> = HashMap::new();

        for acc in accounts.into_iter() {
            // println!("inserting {:?}", acc);
            Self::insert_account(&mut name_hm, acc);
            // println!("{:?}", name_hm);
        }


        let mut out = Vec::new();

        for (k, v) in name_hm.into_iter(){
            for acc in v {
                let mut accvec = Vec::new();
                accvec.push(k.clone());
                accvec.extend(acc.into_iter());
                out.push(accvec);
            }
        }

        out
    }

    fn insert_account(hm: &mut HashMap<String, Vec<BTreeSet<String>>>, account: Vec<String>){
        let mut acc_iter = account.into_iter();
        let name = acc_iter.next().unwrap();
        let emails = acc_iter.collect();

        if hm.contains_key(&name){
            let entry = hm.remove(&name).unwrap();
            hm.insert(name, Self::insert_emails(entry, emails));
        } else {
            hm.insert(name, vec![emails.into_iter().collect()]);
        }
    }

    fn insert_emails(x: Vec<BTreeSet<String>>, emails: Vec<String>) -> Vec<BTreeSet<String>>{
        let mut matched = BTreeSet::<String>::new();
        let mut unmatched = Vec::new();


        's: for subs in x.into_iter() {
            for email in emails.iter() {
                if subs.contains(email) {
                    matched.extend(subs);
                    continue 's;
                }
            }
            unmatched.push(subs);
            continue 's;
        }
        matched.extend(emails.into_iter());
        unmatched.push(matched);
        unmatched
    }
}

fn main() {
    let v: Vec<Vec<String>> = vec![
        strvec!["John","johnsmith@mail.com","john_newyork@mail.com"],
        strvec!["John","johnsmith@mail.com","john00@mail.com"],
        strvec!["Mary","mary@mail.com"],
        strvec!["John","johnnybravo@mail.com"]
    ];

    println!("{:?}\n\n", Solution::accounts_merge(v));


    let v2 = vec![
        strvec!["David","David0@m.co","David1@m.co"],
        strvec!["David","David3@m.co","David4@m.co"],
        strvec!["David","David4@m.co","David5@m.co"],
        strvec!["David","David2@m.co","David3@m.co"],
        strvec!["David","David1@m.co","David2@m.co"]
    ];
    println!("{:?}", Solution::accounts_merge(v2));
}
