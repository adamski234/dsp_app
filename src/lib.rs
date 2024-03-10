#![allow(clippy::needless_return)]

mod signals;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct SignalProcessor {
	/// Sampling frequency in Hz
	pub sampling_frequency: f64,
	/// Starting time offset in s
	pub starting_time: f64,
	signals: Vec<Box<dyn signals::CalculableSignal>>
}

#[wasm_bindgen]
pub struct CoordPair {
	/// Time in seconds
	pub x: f64,
	/// Value of signal
	pub y: f64,
}

#[wasm_bindgen]
impl SignalProcessor {
	pub fn new(sampling_frequency: f64, starting_time: f64) -> Self {
		return Self {
			sampling_frequency,
			starting_time,
			signals: Vec::new(),
		};
	}

	pub fn add_sine(&mut self, signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, phase_shift: f64) {
		self.signals.push(Box::new(signals::SineSignal::new(signal_freq, duration, start_offset, amplitude, phase_shift)));
	}

	pub fn add_half_wave_rectified_sine(&mut self, signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, phase_shift: f64) {
		self.signals.push(Box::new(signals::HalfWaveRectifiedSineSignal::new(signal_freq, duration, start_offset, amplitude, phase_shift)));
	}

	pub fn add_full_wave_rectified_sine(&mut self, signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, phase_shift: f64) {
		self.signals.push(Box::new(signals::FullWaveRectifiedSineSignal::new(signal_freq, duration, start_offset, amplitude, phase_shift)));
	}

	pub fn add_uniform_noise(&mut self, duration: f64, start_offset: f64, amplitude: f64) {
		self.signals.push(Box::new(signals::UniformNoise::new(duration, start_offset, amplitude)));
	}

	pub fn add_normal_noise(&mut self, duration: f64, start_offset: f64, amplitude: f64) {
		self.signals.push(Box::new(signals::NormalNoise::new(duration, start_offset, amplitude)));
	}

	pub fn add_rectangular(&mut self, signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, duty_cycle: f64) {
		self.signals.push(Box::new(signals::RectangularSignal::new(signal_freq, duration, start_offset, amplitude, duty_cycle)));
	}

	pub fn add_symmetric_rectangular(&mut self, signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, duty_cycle: f64) {
		self.signals.push(Box::new(signals::SymmetricRectangularSignal::new(signal_freq, duration, start_offset, amplitude, duty_cycle)));
	}

	pub fn add_triangular(&mut self, signal_freq: f64, duration: f64, start_offset: f64, amplitude: f64, duty_cycle: f64) {
		self.signals.push(Box::new(signals::TriangularSignal::new(signal_freq, duration, start_offset, amplitude, duty_cycle)));
	}

	pub fn add_unit_jump(&mut self, flip_offset: f64, duration: f64, start_offset: f64, amplitude: f64) {
		self.signals.push(Box::new(signals::UnitJump::new(flip_offset, duration, start_offset, amplitude)));
	}

	pub fn add_unit_pulse(&mut self, time_offset: f64, duration: f64, start_offset: f64, amplitude: f64) {
		self.signals.push(Box::new(signals::UnitPulse::new(time_offset, duration, start_offset, amplitude)));
	}

	pub fn add_unit_noise(&mut self, probability: f64, duration: f64, start_offset: f64, amplitude: f64) {
		self.signals.push(Box::new(signals::UnitNoise::new(probability, duration, start_offset, amplitude)));
	}

	pub fn get_signal(&self) -> Vec<CoordPair> {
		let signal_duration = self.signals.iter().map(|signal| signal.get_signal_end()).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
		let ending_point = self.starting_time + signal_duration; // in seconds
		let sampling_points = linspace_by_freq(self.starting_time, ending_point, self.sampling_frequency);
		return self.signals[0].calculate_signal(&sampling_points);
	}
}

/// `starting_point` is inclusive seconds
/// `end_point` is exclusive seconds
/// `freq` in in Hz
#[wasm_bindgen]
pub fn linspace_by_freq(starting_point: f64, end_point: f64, freq: f64) -> Vec<f64> {
	let step = freq.recip();
	let points = ((end_point - starting_point) / step).floor() as usize;
	return (0..points).map(|offset| starting_point + (offset as f64 * step)).collect();
}