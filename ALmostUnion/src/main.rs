//Almost union 
//Dedicated to Noemi!

use std::{io::{self, prelude::*}, fmt};

fn main() {
    let mut buf: String = String::with_capacity(100);


    //get input as string and split
    io::stdin().read_to_string(&mut buf).expect("err");
    let lines: Vec<&str> = buf.split('\n').collect();

    
    //loop through 
    let mut l = 0;
    while l < lines.len() {
        if lines[l] == "" {
            l += 1;
            continue;
        }

        

        let mut first = lines[l].split_whitespace();

        //go to next and unwrap 
        let n: usize = first.next().unwrap().parse().unwrap(); 
        let m: usize = first.next().unwrap().parse().unwrap();

        let mut auf = AlmostUnionFind::new(n);

        

        //loop through and match first value to function 
        for i in (l + 1)..(l + m + 1) {
            let line = lines[i].split_whitespace()
                .map(|_value| _value.parse::<usize>().ok().unwrap())
                .collect::<Vec<usize>>();

            match line[0] {
                1 => auf.union(line[1], line[2]),
                2 => auf._move(line[1], line[2]),
                3 => {
                    let (size, sum) = auf._return(line[1]);
                    println!("{} {}", size, sum);
                },
                _ => panic!(),
            }
        }
        l += m + 1;
    }
}



#[derive(Debug)]
/// ## AlmostUnionFind
/// Thanks to dear Katis everything is in main which takes away the coolest thing about Rust

///We set the size and id of each set
/// id is the parent node of i, if set id is i, i is a root 
struct AlmostUnionFind {
    n: usize,

    set_size: Vec<usize>,
    set_id: Vec<usize>,
    set_sum: Vec<usize>,
}

///Implementation
/// Returns a new AUS of size n

impl AlmostUnionFind {
    fn new(n: usize) -> AlmostUnionFind {
        let set_size = vec![1;2*(n+1)];
        let mut set_id = vec![0;2*(n+1)];
        let mut set_sum = vec![0;2*(n+1)];

        let mut j = n+2;

        //go through the roots till n
        for i in 1..=n {
            set_id[i] = j;
            set_id[j] = j;
            set_sum[j] = i;
            j += 1;
        }

        AlmostUnionFind {  n, set_size, set_id, set_sum }
    }

    /// ## find
    /// Function that finds the root of a set
    /// While is not equal to itself 
    /// Afterwards it Compresses its path through the chain
    fn find(&mut self, mut p: usize) -> usize {
        let mut root: usize = p;

        while root != self.set_id[root] {
            root = self.set_id[root];
        }

        while p != root {
            let next: usize = self.set_id[p];
            self.set_id[p] = root;
            p = next;
        }
        root
    }
    /// ## union
    /// Function that unions the sets containing p and q
    fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q); 

        // If they're not already in the same set
        if root_p != root_q {
            self.set_size[root_q] += self.set_size[root_p];
            self.set_sum[root_q] += self.set_sum[root_p];
            self.set_id[root_p] = root_q;
        }
    }

    /// Oops I did it again 
    /// I used underline, 
    /// But it was necessary 
    /// To avoid confusion

    /// ## move
    /// Moves element p into the set containing q
    fn _move(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        // Check if they are not in the same set
        //Not setters and getters but set as a group of stuff 
        if root_p != root_q {
            self.set_size[root_q] += 1;
            self.set_size[root_p] -= 1;
            self.set_sum[root_q] += p;
            self.set_sum[root_p] -= p;

            self.set_id[p] = root_q;
        }
    }


    /// ## return
    /// Function that returns the size of the set containing p
    /// And the sum of all elements
    fn _return(&mut self, p: usize) -> (usize, usize) {
        let root_p = self.find(p);
        let size = self.set_size[root_p];
        let sum = self.set_sum[root_p];
        (size, sum)
    }
}


impl fmt::Display for AlmostUnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Set Size: {:?}\nSet Id: {:?}\nSet Bu: {:?}\nSet Sum: {:?}", 
        &self.set_size[self.n+2..], 
        &self.set_id[1..self.n+1], 
        &self.set_id[self.n+2..], 
        &self.set_sum[self.n+2..])
    }
}