# Peacock

**Peacock** is a Rust library for **printing coloured text** to make your CLI app  beautiful as a Peacock

## Features

- **Easy to use** Apply modification inside string
- **Pre defined Colors** Common colors  are already defined
- **Use RGB colors** Apply colors using RGB value

## Usage

Add Peacock to your Cargo.toml:

```toml
[dependencies]
peacock = "0.1"
```

Add Peacock using Cargo CLI:
```
cargo add Peacock
```

### Example

```rust
use peacock::pealn;

fn main() {
    //using predefined and RGB color
      pealn!("Name:[yellow](John Doe)\nAge:[(218, 66, 245)](23)");
}
```
---

*Peacock makes your CLI apps more colorful and expressive!*