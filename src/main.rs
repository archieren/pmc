#![allow(unused_imports)]
use parallel_maximal_clique::graph::{Graph, HashMapGraph as H_Graph, Kcores};
use rayon::prelude::*;

struct Person {
    age: u32,
}

pub fn it_works() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
    return;
}

fn main() {
    it_works();
    let mut h_graph = H_Graph::new();

    println!("{}", h_graph.is_empty());
    let nodes: Vec<usize> = vec![10, 122, 33, 88, 99, 1000, 99];
    for node in nodes.iter() {
        let res = h_graph.add_node(*node);
        match res {
            Ok(_) => println!("add {} ok!", node),
            Err(_) => println!("add {} failed. It maybe a duplicated case!", node),
        };
    }
    let ids = h_graph.ids();
    for val in ids {
        println!("{}", val)
    }

    let mut kscore = Kcores::new(&h_graph);
    let _ = kscore.cores_by_batagelj();

    h_graph.has_id(99);
}
