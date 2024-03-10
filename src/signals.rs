use std::f64::consts::TAU;

use rand::{distributions::Distribution, Rng};

pub trait CalculableSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair>;
	fn get_signal_end(&self) -> f64;
}

pub struct SineSignal {
	/// Frequency in Hz
	signal_freq: f64,
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
	/// Phase shift in radians
	phase_shift: f64,
}

impl CalculableSignal for SineSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return sampling_points.iter().map(|point| {
			return crate::CoordPair {
				x: *point,
				y: self.amplitude * (self.signal_freq * TAU * point + self.phase_shift).sin()
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl SineSignal {
	pub fn new(signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, phase_shift: f64) -> Self {
		return Self {
			signal_freq,
			duration,
			start_offset,
			amplitude,
			phase_shift,
		};
	}
}

pub struct HalfWaveRectifiedSineSignal {
	/// Sine signal source to rectify
	inner_sine: SineSignal,
}

impl CalculableSignal for HalfWaveRectifiedSineSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return self.inner_sine.calculate_signal(sampling_points).into_iter().map(|mut sample| {
			sample.y = sample.y.clamp(0.0, f64::MAX);
			return sample;
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.inner_sine.start_offset + self.inner_sine.duration;
	}
}

impl HalfWaveRectifiedSineSignal {
	pub fn new(signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, phase_shift: f64) -> Self {
		return Self {
			inner_sine: SineSignal {
				signal_freq,
				duration,
				start_offset,
				amplitude,
				phase_shift,
			}
		};
	}
}

pub struct FullWaveRectifiedSineSignal {
	/// Sine signal source to rectify
	inner_sine: SineSignal,
}

impl CalculableSignal for FullWaveRectifiedSineSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return self.inner_sine.calculate_signal(sampling_points).into_iter().map(|mut sample| {
			sample.y = sample.y.abs();
			return sample;
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.inner_sine.start_offset + self.inner_sine.duration;
	}
}

impl FullWaveRectifiedSineSignal {
	pub fn new(signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, phase_shift: f64) -> Self {
		return Self {
			inner_sine: SineSignal {
				signal_freq,
				duration,
				start_offset,
				amplitude,
				phase_shift,
			}
		};
	}
}

pub struct UniformNoise {
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
}

impl CalculableSignal for UniformNoise {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return sampling_points.iter().zip(rand::distributions::Uniform::new(-self.amplitude, self.amplitude).sample_iter(rand::thread_rng())).map(|(point, value)| {
			return crate::CoordPair {
				x: *point,
				y: value
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl UniformNoise {
	pub fn new(duration: f64, start_offset: f64, amplitude: f64) -> Self {
		return Self {
			duration,
			start_offset,
			amplitude,
		};
	}
}

pub struct NormalNoise {
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
}

impl CalculableSignal for NormalNoise {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return sampling_points.iter().zip(rand::thread_rng().sample_iter(rand_distr::StandardNormal)).map(|(point, value): (_, f64)| {
			return crate::CoordPair {
				x: *point,
				y: value * self.amplitude
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl NormalNoise {
	pub fn new(duration: f64, start_offset: f64, amplitude: f64) -> Self {
		return Self {
			duration,
			start_offset,
			amplitude,
		};
	}
}

pub struct RectangularSignal {
	/// Sine signal source to rectify
	inner_signal: SymmetricRectangularSignal,
}

impl CalculableSignal for RectangularSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return self.inner_signal.calculate_signal(sampling_points).into_iter().map(|mut point| {
			point.y = point.y.clamp(0.0, f64::MAX);
			return point;
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.inner_signal.start_offset + self.inner_signal.duration;
	}
}

impl RectangularSignal {
	pub fn new(signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, duty_cycle: f64) -> Self {
		return Self {
			inner_signal: SymmetricRectangularSignal {
				signal_freq,
				duration,
				start_offset,
				amplitude,
				duty_cycle,
			}
		};
	}
}

pub struct SymmetricRectangularSignal {
	/// Frequency in Hz
	signal_freq: f64,
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
	/// Part of each period where signal is high, between 0 and 1
	duty_cycle: f64,
}

impl CalculableSignal for SymmetricRectangularSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		let function_period = 1.0 / self.signal_freq;
		let flip_point_within_period = function_period * self.duty_cycle;
		return sampling_points.iter().map(|point| {
			let offset_within_period = point % function_period;
			return crate::CoordPair {
				x: *point,
				y: if offset_within_period > flip_point_within_period { -self.amplitude } else { self.amplitude },
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl SymmetricRectangularSignal {
	pub fn new(signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, duty_cycle: f64) -> Self {
		return Self {
			signal_freq,
			duration,
			start_offset,
			amplitude,
			duty_cycle,
		};
	}
}

pub struct TriangularSignal {
	/// Frequency in Hz
	signal_freq: f64,
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
	/// Part of each period where signal is high, between 0 and 1
	duty_cycle: f64,
}

impl CalculableSignal for TriangularSignal {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		let function_period = self.signal_freq.recip();
		let flip_point_within_period = function_period * self.duty_cycle;
		return sampling_points.iter().map(|point| {
			let offset_within_period = point % function_period;
			// triangle wave has two parts - before flip point and after flip point. Flip point does not need to be in the middle
			// this expresses how far the point is in the part between period start and flip point or flip point and period end
			// second part needs to be in reverse - subtracted from 0 because it needs to go down; reverse of the first part
			let part_of_wave_side = if offset_within_period > flip_point_within_period { (offset_within_period - flip_point_within_period) / (function_period - flip_point_within_period) } else { 1.0 - offset_within_period / flip_point_within_period };
			return crate::CoordPair {
				x: *point,
				y: part_of_wave_side * self.amplitude
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl TriangularSignal {
	pub fn new(signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, duty_cycle: f64) -> Self {
		return Self {
			signal_freq,
			duration,
			start_offset,
			amplitude,
			duty_cycle,
		};
	}
}

pub struct UnitJump {
	/// Time when signal changes from 0 to 1, in seconds relative to local starting point
	flip_offset: f64,
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
}

impl CalculableSignal for UnitJump {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		// required because sampling points refer to global time, not local
		let global_flip_point = self.start_offset + self.flip_offset;
		return sampling_points.iter().map(|point| {
			return crate::CoordPair {
				x: *point,
				y: if *point > global_flip_point { self.amplitude } else { 0.0 }
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl UnitJump {
	pub fn new(flip_offset: f64, duration: f64, start_offset: f64, amplitude: f64) -> Self {
		return Self {
			flip_offset,
			duration,
			start_offset,
			amplitude,
		};
	}
}

pub struct UnitPulse {
	/// Time when signal changes from 0 to 1, in seconds relative to local starting point
	/// Will snap to the closest measurement point
	time_offset: f64,
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
}

impl CalculableSignal for UnitPulse {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		// required because sampling points refer to global time, not local
		let global_flip_point = self.start_offset + self.time_offset;
		// needs to be absolute - positive
		let mut smallest_diff_so_far = f64::MAX;
		let mut smallest_diff_time = 0.0;

		for point in sampling_points {
			let current_diff = (global_flip_point - *point).abs();
			if current_diff < smallest_diff_so_far {
				smallest_diff_so_far = current_diff;
				smallest_diff_time = *point;
			}
		}

		return sampling_points.iter().map(|point| {
			return crate::CoordPair {
				x: *point,
				y: if *point == smallest_diff_time { self.amplitude } else { 0.0 }
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl UnitPulse {
	pub fn new(time_offset: f64, duration: f64, start_offset: f64, amplitude: f64) -> Self {
		return Self {
			time_offset,
			duration,
			start_offset,
			amplitude,
		};
	}
}

pub struct UnitNoise {
	/// Probability for signal to be amplitude. Between 0 and 1
	probability: f64,
	/// Duration in s
	duration: f64,
	/// Starting time in s relative to global starting point
	start_offset: f64,
	/// Dimensionless amplitude
	amplitude: f64,
}

impl CalculableSignal for UnitNoise {
	fn calculate_signal(&self, sampling_points: &[f64]) -> Vec<crate::CoordPair> {
		return sampling_points.iter().zip(rand_distr::Bernoulli::new(self.probability).unwrap().sample_iter(rand::thread_rng())).map(|(point, value)| {
			return crate::CoordPair {
				x: *point,
				y: if value { self.amplitude } else { 0.0 },
			};
		}).collect();
	}
	fn get_signal_end(&self) -> f64 {
		return self.start_offset + self.duration;
	}
}

impl UnitNoise {
	pub fn new(probability: f64, duration: f64, start_offset: f64, amplitude: f64) -> Self {
		return Self {
			probability,
			duration,
			start_offset,
			amplitude,
		};
	}
}