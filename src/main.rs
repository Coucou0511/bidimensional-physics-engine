use crate::Object_type::Circle;

// each object will be a struct with the object's x&y position, x&y velocity, angular velocity, mass, applied force, angle at which the force is applied, and hitbox
#[derive(Clone, Debug)]
struct Object {
    x_pos: f64, // in m
    y_pos: f64,
    x_vel: f64, // in m/s
    y_vel: f64,
    ang_pos: f64, // in rad
    ang_vel: f64, // in rad/s
    mass: f64, //in kilograms
    force: f64, //in N
    f_ang: f64, // in rad
    obj_type: Object_type,
}
impl Object {
    fn apply_grav_pull_from(&mut self, target: Object) {
        let rela_pos = (self.x_pos - target.x_pos, self.y_pos - target.y_pos); // relative x and y position of the target to the object we are computing for

        let G: f64 = 0.00000000006674; // gravitationnal constant in N*m²/kg²
        let r = ((rela_pos.0).powf(2.0) + (rela_pos.1).powf(2.0)).sqrt(); // distance between the centers of the two objects
        let F = G*((self.mass*target.mass)/r.powf(2.0)); // gravitationnal pull in N
        let angle = (rela_pos.1).atan2(rela_pos.0); // angle at which the gravitationnal pull is applied to relative to the x axis
        
        let new_force_vector = (F*(angle).cos() + self.force*(self.f_ang).cos(), F*(angle).sin() + self.force*(self.f_ang).sin()); // x and y values of the new force vector
        
        self.force = (new_force_vector.0.powf(2.0) + new_force_vector.1.powf(2.0)).sqrt(); // update the force values of the object
        self.f_ang = (new_force_vector.1).atan2(new_force_vector.0);
    }
}

#[derive(Clone, Debug)]
enum Object_type {
    Circle(f64), // a perfect circle of radius f64 in meters
}

fn main() {
    let mut earth = Object { // creating an object to the size of the Earth at 0, 0
        x_pos : 0.0,
        y_pos : 0.0,
        x_vel : 0.0,
        y_vel : 0.0,
        ang_pos : 0.0,
        ang_vel : 0.0,
        mass : 5.9722*10.0_f64.powf(24.0),
        force : 0.0,
        f_ang : 0.0,
        obj_type : Circle(6.371*10.0_f64.powf(6.0)),
    };

    let mut moon = Object { // creating an object the size of the moon, placed at the average distance from earth
        x_pos : 0.0,
        y_pos : -3.631*10.0_f64.powf(8.0),
        x_vel : 0.0,
        y_vel : 0.0,
        ang_pos : 0.0,
        ang_vel : 0.0,
        mass : 7.35*10.0_f64.powf(22.0),
        force : 0.0,
        f_ang : 0.0,
        obj_type : Circle(1.74*10.0_f64.powf(6.0)),
    };

    // quick demo of the current capability 
    println!("The earth is at position x=0.0m, y=0.0m, weighs 5.9722e24kg, and has no force applied to it.");
    println!("\nThe moon is at position x=0.0m, y=-363100000.0m, weighs 7.349999999999999e22kg, and has no force applied to it.");

    println!("\n\nLet's apply Newton’s Law of Universal Gravitation !");

    moon.apply_grav_pull_from(earth.clone());
    earth.apply_grav_pull_from(moon.clone());

    println!("\n\nEarth now has a pull of {}N at an angle of {}rad relative to the x plane.", earth.force, earth.f_ang);
    println!("\nMoon now has a pull of {}N at an angle of {}rad relative to the x plane.", moon.force, moon.f_ang);
}
