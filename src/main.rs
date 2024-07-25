use nalgebra::Vector2;

fn main() {
    println!("Hello, world!");
}

pub struct Spot{
    position: Vector2<f32>,
    color: f32,
    std_dev: f32
}

pub fn calc_pixel(position: Vector2<f32>, spots: Vec<Spot>) -> f32{
    spots.iter().map(|spot| {
        ((spot.position-position).magnitude_squared()*spot.std_dev*-1.0).exp()*spot.color
    }).sum::<f32>()
}
pub fn calc_grads(position: Vector2<f32>, spots: Vec<Spot>) -> [f32; 4]{
    spots.iter().fold([0.0; 4], |acc, spot| {
        let diff = spot.position - position;
        let neg_mag_sq = -diff.magnitude_squared();
        let exp = (spot.std_dev*neg_mag_sq).exp();
        let result = exp*spot.color;
        let color_grad = exp;
        let std_dev_grad = result*neg_mag_sq;
        let x_grad = result*-2.0*spot.std_dev*diff.x;
        let y_grad = result*-2.0*spot.std_dev*diff.y;
        [acc[0]+x_grad, acc[1]+y_grad, acc[2]+color_grad, acc[3]+std_dev_grad]
    })
}