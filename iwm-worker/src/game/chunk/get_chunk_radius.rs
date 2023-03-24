use crate::game::player::player_struct::Vec2;

pub(crate) fn get_chunk_radius(mut x: i32, mut z: i32, radius: u8) -> Vec<Vec2<i32>> {
    let points = (radius * 2) + 2;
    let mut use_plus = true;
    let mut array_vec2 = Vec::new();

    array_vec2.push(Vec2 { x, z });
    
    for index in 0..points {
        use_plus = !use_plus;

        for height in 0..index {
            if height == points - 2 {
                break;
            }
            if use_plus {
                x += 1
            } else {
                x -= 1
            }

            array_vec2.push(Vec2 { x, z })
        }

        if index == points - 1 {
            break;
        }

        for _ in 0..points {
            if use_plus {
                z += 1
            } else {
                z -= 1
            }

            array_vec2.push(Vec2 { x, z })
        }
    }

    array_vec2
}
