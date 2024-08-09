
pub struct Projectile{
    pub v_o:f64, 
    pub angle:f64,
    pub grav:f64
}

// Modelling of a projectile
impl Projectile{
    
    //Horizontal positioning
    pub fn hp(&self, time_i:i32) -> f64{
        let time = time_i as f64;
        return self.v_o * self.angle.to_radians().cos() * time as f64;
    }
    
    //Vertical positioning
    pub fn vp(&self, time_i:i32) -> f64{
        let time = time_i as f64;
        return self.v_o * self.angle.to_radians().sin() * time - (0.5 * self.grav * time  * time);
    }
    
    pub fn hv(&self) -> f64{
    
        return self.v_o * self.angle.to_radians().sin();
    }
    
    pub fn vv(&self, time_i:f64) -> f64{
        let time = time_i as f64;
        return self.v_o * self.angle.to_radians().sin() * time - (0.5 * self.grav * time * time);
    }
}

