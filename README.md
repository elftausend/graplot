# graplot

[![Crates.io version](https://img.shields.io/crates/v/graplot.svg)](https://crates.io/crates/graplot)
[![Docs](https://docs.rs/graplot/badge.svg?version=0.1.3)](https://docs.rs/graplot/0.1.3/graplot/)

Experimental plotting library written in Rust and based on [macroquad].

[macroquad]: https://github.com/elftausend/macroquad

## [Examples]

[Examples]: https://github.com/elftausend/graplot/tree/main/examples

```rust
use graplot::Plot;

let plot = Plot::new([-4., -2., 1., 4.]);
plot.show();
```

![plot1](pictures/plot1.png)

Multiple graphs:
```rust
use graplot::Plot;

let xs = [1., 2., 3.,];
let ys = [1.7, 3., 1.9];

let ys1 = [1.4, 1.6, 1.5];    

let ys2 = [0.9, 1.2, 1.7, 1.9, 2.];    

let mut plot = Plot::new((xs, ys));
plot.add((xs, ys1, "c-o"));
plot.add((ys2, "r-"));
plot.show();
```

![multiple graphs](pictures/multiple.png)


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
// or alternatively: let plot = Plot::new((|x: f64| x.sin(), x(4.)));
let plot = Plot::new((xs, ys));
plot.show();
```

![Sinewave](pictures/sine_wave.png)


x³ + x² - 0.08:
```rust
use graplot::{Plot, x};

// x(...) ... sets the absolute max value for x 
let plot = Plot::new((|x: f64| x.powf(3.) + x.powf(2.) - 0.08, x(1.)) );
plot.show();
```
![pol3](pictures/pol3.png)

x² - 0.5:
```rust
use graplot::Plot;

let plot = Plot::new(|x: f64| x.powf(2.) - 0.5);
plot.show();
```
![squared](pictures/x2.png)

Using a line description: (matplotlib)

```rust
use graplot::Plot;

// c ... cyan color, - ... solid line, o ... ring marker
let plot = Plot::new(([-4., -3., -3.4, -3.75, -4.1], "c-o"));
plot.show();
```

![line_desc](pictures/line_desc.png)