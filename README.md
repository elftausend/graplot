# graplot

Experimental plotting library written in Rust and based on [macroquad].

[macroquad]: https://github.com/elftausend/macroquad

```rust
use graplot::Plot;

let plot = Plot::new([-4., -2., 1., 4.]);
plot.show();
```
![plot1](plot1.png)

Sine wave:
```rust
use graplot::Plot;

let mut xs = [0.; 1000]; 

let mut add = 0f64;
for idx in 0..1000 {
    xs[idx] = add/1000.;
    add += 1.;
}
    
let mut ys = [0.; 1000];
for (i, y) in ys.iter_mut().enumerate() {
    *y = (2. * std::f64::consts::PI * xs[i]).sin();
}

let plot = Plot::new((xs, ys));
plot.show();
```

![Sinewave](sine_wave.png)


x³ + x² - 0.08:
```rust
use graplot::Plot;

let plot = Plot::new((|x: f64| x.powf(3.) + x.powf(2.) - 0.08, 10000) );
plot.show();
```
![pol3](pol3.png)

x² - 0.5:
```rust
use graplot::Plot;

let plot = Plot::new(|x: f64| x.powf(2.) - 0.5);
plot.show();
```
![squared](x2.png)