use crate::ObjectType::Circle;

// each object will be a struct with the object's x&y position, x&y velocity, angular velocity, mass, applied force, angle at which the force is applied, and hitbox
#[derive(Clone, Debug)]
struct Object {
    x_pos: f64, // in m
    y_pos: f64,
    x_vel: f64, // in m/s
    y_vel: f64,
    mass: f64, //in kilograms
    force_x: f64,
    force_y: f64,
    obj_type: ObjectType,
}
impl Object {
    fn clear(&mut self) {
        self.force_x = 0.0;
        self.force_y = 0.0;
    }
    fn compute_vel(&mut self) {
        self.x_vel = self.force_x / self.mass;
        self.y_vel = self.force_y / self.mass;
    }
    fn apply_pos(&mut self) {
        self.x_pos = self.x_pos + self.x_vel;
        self.y_pos = self.y_pos + self.y_vel;
    }
}

#[derive(Clone, Debug)]
enum ObjectType {
    Circle(f64), // a perfect circle of radius f64 in meters
}

fn apply_grav_pull(object1: &mut Object, object2: &mut Object) {
    let rela_pos = (object2.x_pos - object1.x_pos, object2.y_pos - object1.y_pos);

    const G: f64 = 6.674e-11; // gravitationnal constant in N*m²/kg²
    let r_squared = (rela_pos.0).powi(2) + (rela_pos.1).powi(2); // distance between the centers of the two objects
    let r = r_squared.sqrt();
    if r == 0.0 {
        return;
    }
    let f = G*((object1.mass*object2.mass)/r_squared); // gravitationnal pull in N

    let force_x = f * rela_pos.0 / r;
    let force_y = f * rela_pos.1 /r;

    object1.force_x += force_x;
    object1.force_y += force_y;
    object2.force_x -= force_x;
    object2.force_y -= force_y;
}

fn main() {
    let mut earth = Object { // creating an object to the size of the Earth at 0, 0
        x_pos : 0.0,
        y_pos : 0.0,
        x_vel : 0.0,
        y_vel : 0.0,
        mass : 9999999999.0,
        force_x : 0.0,
        force_y : 0.0,
        obj_type : Circle(6.0),
    };

    let mut moon = Object { // creating an object the size of the moon, placed at the average distance from earth
        x_pos : 5.0,
        y_pos : 0.0,
        x_vel : 0.0,
        y_vel : 0.0,
        mass : 9999999999.0,
        force_x : 0.0,
        force_y : 0.0,
        obj_type : Circle(1.0),
    };

    // quick demo of the current capability 
    for i in 0..6 {
        println!("{}", moon.x_pos);
        earth.clear();
        moon.clear();
        apply_grav_pull(&mut earth, &mut moon);
        earth.compute_vel();
        moon.compute_vel();
        earth.apply_pos();
        moon.apply_pos();

    }

}
