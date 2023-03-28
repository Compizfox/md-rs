use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use flate2::write::GzEncoder;
use flate2::Compression;
use rayon::prelude::*;

use crate::types::Particle;
use crate::N_PARTICLES;

pub struct XYZWriter {
    io_tx: Option<Sender<Vec<u8>>>,
    pub h: Option<thread::JoinHandle<()>>,
}

impl XYZWriter {
    pub fn new(path: &str) -> Self {
        let (io_tx, io_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

        // Spawn IO thread
        let p = path.to_owned();
        let h = thread::spawn(move || {
            let mut bufwriter = BufWriter::with_capacity(
                8 * 1024,
                GzEncoder::new(File::create(p).unwrap(), Compression::best()),
            );

            for line in io_rx {
                bufwriter.write_all(&line).unwrap();
            }

            bufwriter.flush().expect("Could not flush output buffer");
            bufwriter
                .into_inner()
                .unwrap()
                .finish()
                .expect("Could not finish gzipped output");
        });

        Self { io_tx: Some(io_tx), h: Some(h) }
    }

    pub fn write_frame(&mut self, particles: &[Particle]) {
        let mut line: Vec<u8> = vec![];

        write!(line, "{}\n\n", N_PARTICLES).unwrap();

        line.append(&mut particles
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                format!(
                    "{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n", i,
                    p.position.x, p.position.y, p.position.z,
                    p.velocity.x, p.velocity.y, p.velocity.z
                )
            })
            .reduce(|| String::new(), |a, b| a + &b).into_bytes());

        self.io_tx.as_ref().unwrap().send(line).unwrap();
    }
}

impl Drop for XYZWriter {
    fn drop(&mut self) {
        self.io_tx.take();
        self.h.take().unwrap().join().unwrap();
    }
}
