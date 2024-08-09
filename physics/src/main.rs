mod dep;
use dep::functi::Projectile;
use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    const  VEL:f64 = 60.0;
    const ANGLE:f64 = 60.0;
    const GRAVITY:f64 = 1.62;

    let bullet = Projectile{v_o:VEL, angle:ANGLE, grav:GRAVITY};

    let file_path = "output.csv";
    let mut wtr = Writer::from_path(file_path)?;

    // Write headers
    wtr.write_record(&["X", "Y"])?;



    for second in (0..60).step_by(1){
        let x = bullet.hp(second);
        let y = bullet.vp(second);
        
        if y < 0.0 { 
            let y = 0.0;
            wtr.write_record(&[x.to_string(), y.to_string()])?;
        } else {
            wtr.write_record(&[x.to_string(), y.to_string()])?;
            
        }
        
    }

    wtr.flush()?;

        println!("CSV file created successfully at {}", file_path);

    Ok(())

}

