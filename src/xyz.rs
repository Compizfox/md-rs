use std::fs::File;
use std::io::Write;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::BufWriter;

use crate::N_PARTICLES;
use crate::types::Particle;

pub struct XYZWriter {
    bufwriter: BufWriter<GzEncoder<File>>,
}

impl XYZWriter {
    pub fn new(path: &str) -> Self {
        Self {
            bufwriter: BufWriter::with_capacity(1024 * 1024, GzEncoder::new(File::create(path).unwrap(), Compression::best())),
        }
    }

    pub fn write_frame(&mut self, particles: &[Particle]) {
        write!(self.bufwriter, "{}\n\n", N_PARTICLES).unwrap();
        for (i, p) in particles.iter().enumerate() {
            write!(self.bufwriter, "{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n",
                   i,
                   p.position.x, p.position.y, p.position.z,
                   p.velocity.x, p.velocity.y, p.velocity.z
            ).unwrap();
        }
    }
}

impl Drop for XYZWriter {
    fn drop(&mut self) {
        self.bufwriter.flush().expect("Failed to flush write buffer");
        self.bufwriter.get_mut().try_finish().expect("Failed to finish gzip");
    }
}
