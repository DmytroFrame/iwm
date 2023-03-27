use crate::utils::vec2::Vec2;

pub(crate) fn get_chunk_tracer(mut x: i32, mut z: i32, radius: u8) -> Vec<Vec2<i32>> {
    let mut result = Vec::new();
    let mut is_plus = true;
    let points = (radius * 2) + 2;

    result.push(Vec2 { x, z });

    for index in 0..points {
        is_plus = !is_plus;

        for height in 0..index {
            if height == points - 2 {
                break;
            }

            if is_plus {
                z += 1;
            } else {
                z -= 1;
            }

            result.push(Vec2 { x, z });
        }

        if index == points - 1 {
            break;
        }

        for _ in 0..index {
            if is_plus {
                x += 1;
            } else {
                x -= 1;
            }

            result.push(Vec2 { x, z });
        }
    }

    return result;
}
