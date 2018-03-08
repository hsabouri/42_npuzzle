// fn vec_index_to_array_index(zero_pos: u16, index: u16) -> u16 {
//     if index < zero_pos {
//         index + 1
//     } else if index > zero_pos {
//         index
//     } else {
//         0
//     }
// }

// fn spiral_value_to_vec_index(sq_size: usize, ref table: &Vec<u16>, value: u16) -> u16 {
//     for i in 0..sq_size {
//         if value == table[i] {
//             return i as u16;
//         }
//     }
//     panic!("There is no {} in solved", value);
// }

// pub fn translate_in(sq_size: usize, ref input: &Vec<u16>, ref table: &Vec<u16>) -> Vec<u16> {
//     let mut map: Vec<u16> = vec![0; sq_size];
//     let zero_pos = sq_size / 2;
//     for i in 0..sq_size {
//         map[i] = vec_index_to_array_index(zero_pos as u16, spiral_value_to_vec_index(sq_size, &table, input[i]));
//     }
//     map
// }
