use valora::prelude::*;

/// This exercise is based on https://inconvergent.net/2019/depth-of-field/

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return (1. - t) * v0 + t * v1;
}

fn distance(a: P3, b: P3) -> f32 {
    return ((b.x-a.x).powi(2) + (b.y-a.y).powi(2) + (b.z - a.z).powi(2)).sqrt();
}
fn plane_distance(a: P3, b: P3) -> f32 {
    return (b.z-a.z).abs();
}

fn rnd_sphere(radius: f32, rng: &mut StdRng) -> P3 {
    let inclination = rng.gen::<f32>() * PI * 2. * 4.;
    let azimuth = rng.gen::<f32>() * PI * 2. * 4.;
    return P3::new(
        inclination.sin()*azimuth.cos()*radius,
        inclination.sin()*azimuth.sin()*radius,
        inclination.cos()*radius
    );
}

fn rnd_circle(radius: f32, rng: &mut StdRng) -> P2 {
    let angle = rng.gen::<f32>() * PI * 2. * 4.;
    return P2::new(angle.cos()*radius, angle.sin()*radius);
}

struct Line {
    p1: P3,
    p2: P3,
}

impl Line {
    fn lerp(&self, amt: f32) -> P3 {
		return self.p1.lerp(self.p2, amt);
	}
	
	fn distance(&self) -> f32 {
		return distance(self.p1, self.p2);
	}
}

struct LineDOF {
    lines: Vec<Line>,
    camera: P3,
    focus_distance: f32,
    depth: f32,
}



impl Artist for LineDOF {
    /// Constructs the artist.
    ///
    /// This would be a place to compile any GLSL or construct any expensive
    /// resources needed across the whole composition.
    fn setup(gpu: Gpu, world: World, rng: &mut StdRng) -> Result<Self> {
        let s = world.scale;
        let w = world.width * s;
        let h = world.height * s;
        let depth = 20.;
        let center = world.center();

        // create object
        let mut ld = LineDOF { lines: Vec::new(), camera: P3::new(center.x, center.y, 0.), focus_distance: depth*0.5, depth: depth };
       
        for _ in 0..30 {
            ld.lines.push(Line{
                p1: P3::new(rng.gen::<f32>()*w, rng.gen::<f32>()*h, rng.gen::<f32>()*depth),
                p2: P3::new(rng.gen::<f32>()*w, rng.gen::<f32>()*h, rng.gen::<f32>()*depth),
            })
        }

        return Ok(ld);
    }
    /// Paints a single frame.
    fn paint(&mut self, ctx: Context, canvas: &mut Canvas) {
        if ctx.frame == 0 {
            canvas.set_color(LinSrgb::new(0., 0., 0.));
            canvas.paint(Filled(ctx.world));
        }

        // simply draw the lines
        
        // canvas.set_color_alpha(LinSrgb::new(1., 0., 0.), 1.0);
        // for l in &self.lines {
        //     canvas.move_to(l.p1.xy());
        //     canvas.line_to(l.p2.xy());
        //     canvas.stroke();
        // }
        
        // Note: Setting the alpha value this low never lets it reach full alpha
        canvas.set_color_alpha(LinSrgb::new(1., 1., 1.), 0.001);
        let m = 0.4;
        let e = 5.5;
        // draw the scene dot per dot one line at a time
        for _ in 0..5000 { //30000
            for l in &self.lines {
                // println!("New point");
                // Select a point, v = lerp(a, b, rnd()), on l. 
                let v = l.lerp(ctx.rng.gen::<f32>());
                // Calculate the distance, d = dst(v, c). 
                // let d = distance(self.camera, v);
                let d = plane_distance(self.camera, v);
                // Find the sample radius, r = m * pow(abs(f - d), e). 
                let r = m * (self.focus_distance - d).abs().powf(e);
                // find the new position, w = v + rndSphere(r). 
                // let offset = rnd_sphere(r, ctx.rng);
                let offset = rnd_circle(r, ctx.rng);
                let w = v.xy() + offset.to_vector();

                // Project w into 2D, and draw a pixel/dot.
                // Project by just ignoring the z position
                // draw with line 
                // canvas.move_to(w);
                // canvas.line_to(P2::new(w.x + 1., w.y));
                // canvas.stroke();
                // draw with square Ngon
                canvas.paint(Filled(Ngon::square(w, 1.)));
            }
        }
    }
}

/// This struct (Artist) does roughly the same thing as LineDOF, but prerenders the pixels in a 
struct LineDOFPrerender {
    lines: Vec<Line>,
    camera: P3,
    focus_distance: f32,
    depth: f32,
}



impl Artist for LineDOFPrerender {
    /// Constructs the artist.
    ///
    /// This would be a place to compile any GLSL or construct any expensive
    /// resources needed across the whole composition.
    fn setup(gpu: Gpu, world: World, rng: &mut StdRng) -> Result<Self> {
        let s = world.scale;
        let w = world.width * s;
        let h = world.height * s;
        let depth = 20.;
        let center = world.center();

        // create object
        let mut ld = LineDOFPrerender { lines: Vec::new(), camera: P3::new(center.x, center.y, 0.), focus_distance: depth*0.5, depth: depth };
       
        for _ in 0..30 {
            ld.lines.push(Line{
                p1: P3::new(rng.gen::<f32>()*w, rng.gen::<f32>()*h, rng.gen::<f32>()*depth),
                p2: P3::new(rng.gen::<f32>()*w, rng.gen::<f32>()*h, rng.gen::<f32>()*depth),
            })
        }

        return Ok(ld);
    }
    /// Paints a single frame.
    fn paint(&mut self, ctx: Context, canvas: &mut Canvas) {
        // simply draw the lines
        
        // canvas.set_color_alpha(LinSrgb::new(1., 0., 0.), 1.0);
        // for l in &self.lines {
        //     canvas.move_to(l.p1.xy());
        //     canvas.line_to(l.p2.xy());
        //     canvas.stroke();
        // }
        
        // Note: Setting the alpha value this low never lets it reach full alpha
        canvas.set_color_alpha(LinSrgb::new(1., 1., 1.), 0.001);
        let m = 0.4;
        let e = 5.5;
        // draw the scene dot per dot one line at a time
        for _ in 0..1000 {
            for l in &self.lines {
                // println!("New point");
                // Select a point, v = lerp(a, b, rnd()), on l. 
                let v = l.lerp(ctx.rng.gen::<f32>());
                // Calculate the distance, d = dst(v, c). 
                // let d = distance(self.camera, v);
                let d = plane_distance(self.camera, v);
                // Find the sample radius, r = m * pow(abs(f - d), e). 
                let r = m * (self.focus_distance - d).abs().powf(e);
                // find the new position, w = v + rndSphere(r). 
                // let offset = rnd_sphere(r, ctx.rng);
                let offset = rnd_circle(r, ctx.rng);
                let w = v.xy() + offset.to_vector();
                // Project w into 2D, and draw a pixel/dot.
                // Project by just ignoring the z position
                canvas.move_to(w);
                canvas.line_to(P2::new(w.x + 1., w.y));
                canvas.stroke();
                // Sample a fixed number of times, or according to the length of l. Use a low alpha value, and a high number of samples for a smooth result. 
            }
        }
    }
}

fn main() -> Result<()> {
    run::<LineDOF>(Options::from_args())
}