# application-rs
Rust graphical application starter, uses stdweb for wasm32 & glutin for the rest


```rust
fn main() {
    let config = AppConfig::new("Title Sample",(600,400));
    let mut app = App::new(config);
    app.run(||{

    });
}
```

# Manual FFI without stdweb
Disable default-features (stdw)
add following after wasm instance is created
```js
var callback = function(t){
    results.instance.exports.update(t);
    requestAnimationFrame(callback);
};
window.requestAnimationFrame(callback);
```