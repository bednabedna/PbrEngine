use super::renderer_buffer::*;
use rand::prelude::*;
use rayon::prelude::*;

pub struct PartitionedBuffer {
    pixels: Vec<RgbReal>,
    partitions: Vec<Partition>,
    // active_partitions: Vec<(&'a mut [RgbReal], &'a mut Partition)>,
    partition_width: usize,
    width: usize,
    height: usize,
    min_error: Real,
    target_time: Real,
    debug_error: bool,
    total_partitions_processed: u32,
}

#[derive(Clone)]
struct Partition {
    samples_count: usize,
    error_sums: RgbReal,
    error: Real,
}

impl Partition {
    fn new() -> Partition {
        Partition {
            samples_count: 0,
            error_sums: (0.0, 0.0, 0.0),
            error: 0.0,
        }
    }

    fn clear(&mut self) {
        self.samples_count = 0;
        self.error_sums = (0.0, 0.0, 0.0);
        self.error = 0.0;
    }
}

impl PartitionedBuffer {
    const PARTITION_WIDTH: usize = 16;
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> PartitionedBuffer {
        if width % Self::PARTITION_WIDTH != 0 || height % Self::PARTITION_WIDTH != 0 {
            panic!(format!(
                "buffer width or height is not a multiple of {}",
                Self::PARTITION_WIDTH
            ));
        }
        let partitions_count = (width / Self::PARTITION_WIDTH) * (height / Self::PARTITION_WIDTH);
        let mut partitions = Vec::with_capacity(partitions_count);
        for _ in 0..partitions_count {
            partitions.push(Partition::new());
        }
        PartitionedBuffer {
            pixels: vec![(0.0, 0.0, 0.0); width * height],
            partitions,
            //active_partitions: (0..partitions_count).collect(),
            partition_width: Self::PARTITION_WIDTH,
            width,
            height,
            min_error: 1.0 / (60.0 * 255.0),
            target_time: std::time::Duration::from_millis(12).as_nanos() as Real,
            debug_error: false,
            total_partitions_processed: 0,
        }
    }

    pub fn ratio_processed(&self) -> Real {
        self.total_partitions_processed as Real / self.partitions.len() as Real
    }

    pub fn debug_error(&mut self, v: bool) {
        self.debug_error = v;
    }
}

impl RendererBuffer for PartitionedBuffer {
    fn sample_pixels<'a, F: Fn(usize, usize) -> RgbReal + Send + Sync>(&'a mut self, sampler: F) {
        //let timer = Timer::new("sample_pixels");
        let partition_width = self.partition_width;
        let partions_width = self.width / self.partition_width;
        let partions_height = self.height / self.partition_width;
        let partition_size = (partition_width * partition_width) as Real;
        let error_div = partition_size * 3.0;
        let min_error = self.min_error;
        let sample_start_time = std::time::Instant::now();
        self.total_partitions_processed = self
            .pixels
            .par_chunks_mut(self.partition_width * self.partition_width)
            .zip(self.partitions.par_iter_mut())
            .enumerate()
            .fold_with(
                0u32,
                |total_partitions_processed, (partition_index, (chunk, partition))| {
                    let mut rng = rand::thread_rng();
                    if partition.samples_count <= 10
                        || partition.error > rng.gen::<Real>() * min_error
                    {
                        let buffer_x = (partition_index % partions_width) * partition_width;
                        let buffer_y = (partition_index / partions_height) * partition_width;
                        partition.samples_count += 1;
                        let samples = partition.samples_count as Real;
                        for (local_index, color_sum) in chunk.into_iter().enumerate() {
                            let px = local_index % partition_width;
                            let py = local_index / partition_width;
                            let sampled_color = sampler(buffer_y + py, buffer_x + px);
                            color_sum.0 += sampled_color.0;
                            color_sum.1 += sampled_color.1;
                            color_sum.2 += sampled_color.2;
                            partition.error_sums.0 += color_sum.0 / samples - sampled_color.0;
                            partition.error_sums.1 += color_sum.1 / samples - sampled_color.1;
                            partition.error_sums.2 += color_sum.2 / samples - sampled_color.2;
                        }
                        partition.error = (partition.error_sums.0.abs()
                            + partition.error_sums.1.abs()
                            + partition.error_sums.2.abs())
                            / (samples * error_div);
                        total_partitions_processed + 1
                    } else {
                        partition.error *= 1.02;
                        total_partitions_processed
                    }
                },
            )
            .sum::<u32>();
        if let Some(partition) = self.partitions.first() {
            if partition.samples_count > 10 {
                let time_error = sample_start_time.elapsed().as_nanos() as Real / self.target_time;
                self.min_error = self.min_error * time_error * 0.2 + self.min_error * 0.8;
            }
        }
        //timer.log();
    }

    fn to_img(&self) -> Vec<u8> {
        let partions_width = self.width / self.partition_width;
        let mut img = new_rgbau8_vec(self.width(), self.height());
        // split into rows of partitions
        img.as_mut_slice()
            .par_chunks_mut(self.width * self.partition_width)
            .enumerate()
            .for_each(|(partition_row_index, partition_row)| {
                for p in 0..partions_width {
                    let partition_index = partition_row_index * partions_width + p;
                    let partition = &self.partitions[partition_index];

                    let buffer_offset =
                        partition_index * self.partition_width * self.partition_width;
                    let sc = partition.samples_count as Real;
                    let img_offset = p * self.partition_width;
                    for py in 0..self.partition_width {
                        for px in 0..self.partition_width {
                            let buffer_color =
                                self.pixels[buffer_offset + py * self.partition_width + px];
                            let image_color = &mut partition_row[py * self.width + img_offset + px];
                            image_color.0 = to_channel(if self.debug_error {
                                (300.0 * partition.error).min(1.0)
                            } else {
                                buffer_color.0 / sc
                            });
                            image_color.1 = to_channel(buffer_color.1 / sc);
                            image_color.2 = to_channel(buffer_color.2 / sc);
                        }
                    }
                }
            });
        rgbau8_vec_to_u8_vec(img)
    }

    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }

    fn reset(&mut self) {
        self.pixels.zero_memory();
        for partition in &mut self.partitions {
            partition.clear();
        }
    }
}
