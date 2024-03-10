import init, { SignalProcessor } from "./pkg/signalum.js";

const outputDiv = document.getElementById("output");

init().then(() => {
	const processor = SignalProcessor.new(10000, 0);
	processor.add_unit_noise(0.9, 5, 0, 1);
	const signal = processor.get_signal();
	Plotly.newPlot(outputDiv, [{
		x: signal.map(pair => pair.x),
		y: signal.map(pair => pair.y),
		mode: 'markers',
		type: 'scattergl'
	}])
});