use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MLData {
    pub nodes: Vec<Node>,
    pub tree: Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub i: String,
    #[serde(default = "default_fnz_id")]
    fnz_id: String,
    pub a: HashMap<String, String>,
}

fn default_fnz_id() -> String {
    String::from("-1")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeNode {
    pub i: String,
    pub c: Option<Vec<TreeNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MLDataContainer {
    element_statistics: MLData,
}

fn read_ml_json(path: &Path) -> MLDataContainer{

    let json_str = fs::read_to_string(path).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_str);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    MLDataContainer::deserialize(deserializer).unwrap()
}

fn calc_val(v1: f32, v2:f32) -> Option<f32>{
    if v2 == 0.0{
        None
    }else {
        Some(v1/v2)
    }
}

fn sum_rate(v1: f32, v2:f32, val: f32) -> Option<f32>{
    let rate = calc_val(v1, v2)?;
    //match rate {
    //    Some(r) =>{
    //        Some(r + val)
    //    },
    //    None => {
    //        None
    //    }
    //}

    Some(rate + val)
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


fn get_xx(data:MLDataContainer)->Option<Node>{
    let mut arr = data.element_statistics.nodes; 
    for node in arr.iter(){
        if node.a.contains_key("XX"){
            let val = node.a.get("XX"); 
            match val{
                Some(val)=>{
                    if val.as_str() == "true"{
                        return Some(node.clone());
                    }
                },
                None=>{},
            }
        }
    }
    return None;
}

fn get_vec_hashmaps(data:MLDataContainer)->Vec<HashMap<String,String>>{
    let mut vec = Vec::new();
    for node in data.element_statistics.nodes.iter(){
        vec.push(node.a.clone());
    }
    return vec;
}
 
fn correlation(data1:MLDataContainer, data2:MLDataContainer)->Option<Vec<f64>>{
    let mut node_xx = get_xx(data1);
    match node_xx {
        Some(node_xx)=>{
            let size = (node_xx.a.len()-5) as f64;
            let mut vec_nodes = get_vec_hashmaps(data2);
            let correlations = vec_nodes.iter().map(|f|
            {
                let mut sum = 0.0;
                for (k,v) in node_xx.a.iter(){
                    match k.as_str(){
                        "XX"|"LT"|"TP"|"WH"|"HT" => sum += 0.0,
                        _ => sum += f.iter().filter(|(fk,fv)|*fk == k && *fv == v).count() as f64,
                    }
                }
                sum/size
            }).collect();
            return Some(correlations);
        },
        None =>{
            return None;
        },
    }
}



#[cfg(test)]
mod test{
    use std::path::Path;
    use crate::ml_data::{Person, read_ml_json, get_xx};

    use super::correlation;

    #[test]
    fn json_test(){
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let p: Person = serde_json::from_str(data).unwrap();
        // Do things just like with any other Rust data structure.
        println!("Please call {} at the number {}", p.name, p.phones[0]);
    }


    #[test]
    fn load_json_test(){
        let path = Path::new("resources/1645511997141_M8INRNFV6O_curr.json");
        let data = read_ml_json(&path);
        let node_xx = get_xx(data);
        match node_xx {
            Some(node_xx)=>{
                println!("{:?}", node_xx.a);
            },
            None => {
                println!("No hay ningún nodo con XX")
            },
        }
    }

    #[test]
    fn correlation_test(){
        let path = Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
        let data1 = read_ml_json(&path);
        let data2 = read_ml_json(&path);
        let correlations = correlation(data1, data2);
        match correlations{
            Some(correlations)=>{
                println!("{:?}",correlations);
                let max = correlations.iter().max_by(|a, b| a.total_cmp(&b));
                match  max{
                    Some(max)=>{
                        println!("Correlación máxima = {}",max);
                    },
                    None=>{
                        println!("No hay máximo en el vector de correlaciones");
                    },
                }

            }
            None =>{
                println!("El primer archivo no tiene un elemento XX");
            }
        }
    }
}
