# graplot

[![Crates.io version](https://img.shields.io/crates/v/graplot.svg)](https://crates.io/crates/graplot)
[![Docs](https://docs.rs/graplot/badge.svg?version=0.1.12)](https://docs.rs/graplot/0.1.12/graplot/)

'graplot' is an experimental plotting library written in Rust that is based on [macroquad] (internally [litequad]).
It creates a window displaying the graphs.

[macroquad]: https://github.com/elftausend/macroquad
[litequad]: https://github.com/elftausend/litequad

## Installation

Add 'graplot' as a dependency:
```toml
[dependencies]
graplot = "0.1.12"
```

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

Label the x and y axis and set a title:

```rust
use graplot::{x, Plot};

let mut plot = Plot::new((|x: f64| x.cos(), x(6.)));

plot.set_title("cosine wave");

plot.set_xlabel("x axis");
plot.set_ylabel("y axis");
plot.show();
```
<img src="pictures/cosine_labeled.png" alt="cosine labeled" width="400"/>

[Collatz Conjecture]:

[Collatz Conjecture]: https://github.com/elftausend/graplot/blob/main/examples/collatz.rs

<img src="pictures/collatz.png" alt="collatz conjecture" width="400"/>

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
<img src="pictures/pol3.png" alt="pol3" width="400"/>


x² - 0.5:
```rust
use graplot::Plot;

let plot = Plot::new(|x: f64| x.powf(2.) - 0.5);
plot.show();
```
![squared](pictures/x2.png)

Use the Polynomial struct or polynomial() function to create a polynomial function that runs through all given points:

```rust
use graplot::{x, Plot, Polynomial};

let poly = Polynomial::new(&[2., 3., 1.], &[2., 3., 2.]);
let plot = Plot::new((poly, x(10.)));
plot.show();
```

<img src="pictures/through_points.png" alt="polynomial functions runs through 3 points" width="400"/>

Draw bar graphs:

```rust
use graplot::Bar;

let mut bar = Bar::new(["Ferris", "Stefan", "Test"], &[100., 200., 700.]);
bar.set_title("title");
bar.set_xlabel("test");
bar.show();
```

<img src="pictures/bars.png" alt="bar graph" width="400"/>

Using a line description: (matplotlib)

```rust
use graplot::Plot;

// c ... cyan color, - ... solid line, o ... ring marker
let plot = Plot::new(([-4., -3., -3.4, -3.75, -4.1], "c-o"));
plot.show();
```
![line_desc](pictures/line_desc.png)

Custom scaling:

```rust
use graplot::{Desc, Plot, x};

let mut plot = Plot::new((|x: f64| x.cos(), x(2.)));
plot.set_desc(Desc {
    min_steps_x: 6.,
    spacing_x: 47.,
    ..Default::default()
});
plot.show();
```

Spawning multiple windows on linux (currently not working):

```rust
let mut plot = Plot::new(|x: f64| x.powf(3.) + x.powf(2.) - 0.08);
plot.set_title("x^3 + x^2 - 0.08");
let h = plot.show_threaded() // show_threaded() is currently linux only;

let mut plot = Plot::new(|x: f64| x.powf(2.) + 0.08);
plot.set_title("x²");
plot.show();

h.join().unwrap() // you need to close both windows
```

<img src="pictures/multiple_windows.png" alt="multiple windows" width="500"/>

## Changelog

- 0.1.12: added bar graphs
- 0.1.11: added scatter plots
- 0.1.10: create polynomial functions with a set of points
- 0.1.9: fixed bug
- 0.1.8: set color now uses 3 args, fixed step size
- 0.1.7: Set graph color, custom x & y "line" spacing and step size | "custom scaling"
- 0.1.6: Label x axis, litequad
- 0.1.5: y axis, set title, /*mutliple windows on linux*/ | yanked
- 0.1.4: Multiple graphs