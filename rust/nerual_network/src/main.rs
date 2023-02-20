


fn neuron_output() {
    let inputs: [f32;4] = [1.0,2.0,3.0,2.5];
    let weights:[[f32;4];3] = [[0.2, 0.8, -0.5,1.0],[0.5, -0.91, 0.26, -0.5],[-0.26, -0.27, 0.17, 0.87]];
    let biases: [f32;3] = [2.0,3.0,0.5];
    let mut layer_output: [f32;3] = [0.0;3];

    for (neuron, weights_arr) in weights.iter().enumerate() {
        let mut neuron_output:f32 = 0.0;
        for (index, weight) in weights_arr.iter().enumerate() {
            neuron_output += inputs[index] * weight;
        }
        neuron_output += biases[neuron];
        layer_output[neuron] = neuron_output;
    }
    println!("{:?}", layer_output);

}


use ndarray::{Array1, Array2,Array3, array};

fn dot_product(){
    let inputs = array![1.0,2.0,3.0,2.5];
    let weights = array![[0.2, 0.8, -0.5,1.0],[0.5, -0.91, 0.26, -0.5],[-0.26, -0.27, 0.17, 0.87]];
    let biases = array![2.0,3.0,0.5];


    let result = weights.dot(&inputs) + biases;
    println!("{:?}", result);

}

fn main(){
    // neuron_output();
    dot_product();
}