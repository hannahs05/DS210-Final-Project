extern crate petgraph;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashSet;
use std::collections::HashMap;
//use petgraph::visit::EdgeRef;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
//use petgraph::Graph;

//reading from data file and return a graph structure containing the node index and hash map contaning the original node string
fn read_file(filename: &str) -> (UnGraph<String, ()>,HashMap<NodeIndex,String>){
  //create empty graph and two hash maps
  let mut graph = UnGraph::<String,()>::new_undirected();
  let mut nodeindices = HashMap::new();
  let mut index_to_identifier = HashMap::new();

  //open the file and display error message if file not found
  let path = Path::new(filename);
  let file = File::open(path).expect("file not found");

  //read all lines from the file
  let lines = io::BufReader::new(file).lines();

  //read line by line
  for line in lines{
    if let Ok(edge) = line{
      //parse the edge to vector string
      let nodes: Vec<String> = edge.split_whitespace().map(String::from).collect();

      //if the original input file doesn't have two nodes in one line, skip the line
      if nodes.len() < 2{
        continue;
      }

      //align the first number to node1 and the second number to node2
      let node1 = nodes[0].clone();
      let node2 = nodes[1].clone();

      //if the node1 is not in the hashmap, add it to the hashmap and add it to the graph (this is to ensure there are no duplicate nodes in the graph structure)
      let index1 = *nodeindices.entry(node1).or_insert_with(||{
        let node = nodes[0].clone();
        let index = graph.add_node(node.clone());
        index_to_identifier.insert(index,node);
        index
      });
      //let index1 = *nodeindices.entry(nodes[0].clone()).or_insert_with(|| graph.add_node(nodes[0].clone()));

      //if the node2 is not in the hashmap, add it to the hashmap and add it to the graph
      let index2 = *nodeindices.entry(node2).or_insert_with(||{
        let node = nodes[1].clone();
        let index = graph.add_node(node.clone());
        index_to_identifier.insert(index,node);
        index
      });
      //let index2 = *nodeindices.entry(nodes[1].clone()).or_insert_with(|| graph.add_node(nodes[1].clone()));

      //add the two nodes as an edge to the graph, the third parameter () represents an empty tuple to indicate that there is no additional data to be stored in the edge
      graph.add_edge(index1, index2, ());
    }
  }

  //return the graph and the hashmap
  (graph, index_to_identifier)
}

//compare similarity of two hashsets
fn similarity(set1: &HashSet<NodeIndex>, set2: &HashSet<NodeIndex>) -> f64{
  //find the intersection size of two sets
  let intersection_size = set1.intersection(set2).count() as f64;

  //find the union size of two sets
  let union_size = set1.union(set2).count() as f64;

  //use the quotient from intersection_size and union_size as the similarity score
  intersection_size / union_size
}

//the main can return result enum, it can either be an Ok variant containing a value or an Err variant containing an error
fn main() -> Result<(),Box<dyn std::error::Error>>{

  //collect the command line arguments to vector string
  let args: Vec<String> = env::args().collect();

  //if the number of arguments is less than 3, display an error message and return
  if args.len() < 3{
    eprintln!("Usage: {} <filepath> <similarity_metrics", args[0]);
    return Ok(());
  }

  //align the first argument to the filename and the second argument to the similarity metric
  let filename = &args[1];
  let metric = &args[2];

  //calling the read_file function to read the file and return the graph and the hashmap
  let (graph, _index_to_identifier) = read_file(filename);

  //collect all the indices of the nodes in the graph into a vector
  let nodeindices: Vec<NodeIndex> = graph.node_indices().collect();
  
  //count the number of vertices in the graph
  println!("Graph has {} nodes", graph.node_count());
  /*
  for node in graph.node_indices(){
    let neighbors: Vec<String> = graph.neighbors(node).map(|neighbor| _index_to_identifier[&neighbor].clone()).collect();
    println!("Node {:?} has {:?} edges", _index_to_identifier[&node], neighbors);
  }
*/

  //look through all the node indices, start from 0
  for i in 0..nodeindices.len(){

    //look through all the node indices, start from i+1
    for j in i+1..nodeindices.len(){

      //since there are too many nodes, and it takes a long time to print all the results, I sampled the output whenever i+j divisible by 1000
      if (i+j) % 1000 == 0{

        //find all the neighbors of the for node indice i
        let friendsofi: HashSet<NodeIndex> = graph.neighbors(nodeindices[i]).collect();

        //find all the neighbors of the for node indice j
        let friendsofj: HashSet<NodeIndex> = graph.neighbors(nodeindices[j]).collect();

        //ensure that the similarity value is aligned based on the value of metric, handling the case where an unknown metric is provided gracefully by printing an error message and returning early with Ok
        let similarity_value = match metric.as_str(){
          "jaccard" => similarity(&friendsofi, &friendsofj),
          _ => {

            //print error message
            eprintln!("Unknown metric: {}", metric);
            return Ok(());
          }
        };

        //print similarity score
        println!("Similarity between {:?} and {:?} : {:.2}", nodeindices[i], nodeindices[j], similarity_value);
      }
    }
  }

 Ok(())
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn test_similarity(){
    let set1: HashSet<NodeIndex> = [0,1,2,3,4,5].iter().map(|&x| NodeIndex::new(x)).collect();
    let set2: HashSet<NodeIndex> = [0,1,2,3,4,5,6,7,8,9].iter().map(|&x| NodeIndex::new(x)).collect();

    //similarity value should be 0.6 since 6 numbers are the same from the above two sets and the unit is 10 numbers
    let similarity_value = similarity(&set1, &set2);
    assert_eq!(similarity_value, 0.6);
  }

  #[test]
  fn test_similarity_2(){
    let set1: HashSet<NodeIndex> = [0,1,2,3,4,5].iter().map(|&x| NodeIndex::new(x)).collect();
    let set2: HashSet<NodeIndex> = [3,4,5,6,7,8,9].iter().map(|&x| NodeIndex::new(x)).collect();

    //similarity value should be 0.3 since 3 numbers are the same from the above two sets and the unit is 9 numbers
    let similarity_value = similarity(&set1, &set2);
    assert_eq!(similarity_value, 0.3);
  }
}
