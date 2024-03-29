use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use flate2::write::GzEncoder;
use flate2::Compression;
use rayon::prelude::*;

use crate::types::{Particle, WritableTuple};

/// Write trajectories (XYZ files) asynchronously
pub struct XYZWriter {
    io_tx: Option<Sender<Vec<u8>>>,
    h: Option<thread::JoinHandle<()>>,
}

impl XYZWriter {
    /// Constructs a new `XYZWriter`.
    /// * `path` - Path of the trajectory file to create
    pub fn new(path: &str) -> Self {
        let (io_tx, io_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

        // Spawn IO thread handling compression and IO
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

    /// Dumps a frame, formatting the frame and submitting it to the IO thread
    /// * `particles` - Slice of particles
    pub fn write_frame(&mut self, particles: &[Particle]) {
        let mut line: Vec<u8> = vec![];

        write!(line, "{}\n\n", particles.len()).unwrap();

        line.append(&mut particles
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                format!("{}\t", i)
                    + &p.position.write() + "\t" + &p.velocity.write() + "\n"
            })
            .reduce(String::new, |a, b| a + &b).into_bytes());

        self.io_tx.as_ref().unwrap().send(line).unwrap();
    }
}

impl Drop for XYZWriter {
    fn drop(&mut self) {
        self.io_tx.take();
        self.h.take().unwrap().join().unwrap();
    }
}
