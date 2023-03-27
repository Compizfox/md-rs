use async_compression::tokio::write::GzipEncoder;
use std::io::Write;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;

use crate::N_PARTICLES;
use crate::types::Particle;

pub struct XYZWriter {
    encoder: GzipEncoder<File>,
}

impl XYZWriter {
    pub async fn new(path: &str) -> Self {
        Self {
            encoder: GzipEncoder::new(File::create(path).await.unwrap()),
        }
    }

    pub async fn write_frame(&mut self, particles: &[Particle]) {
        let mut x: Vec<u8> = vec![];
        write!(x, "{}\n\n", N_PARTICLES).unwrap();
        for (i, p) in particles.iter().enumerate() {
            write!(x, "{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n",
                   i,
                   p.position.x, p.position.y, p.position.z,
                   p.velocity.x, p.velocity.y, p.velocity.z
            ).unwrap();
        }
        self.encoder.write_all(&x).await.unwrap();
    }
}

impl Drop for XYZWriter {
    fn drop(&mut self) {
        self.encoder.flush();
    }
}
