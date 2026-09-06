# theemer-rs 
A cool, lightweigh image recoloring tool written in rust. Easily change the theme of any image to your favorite palettes.

---

# Features 
- **Accurate output**: Output is really accurate compared to the original
- **Built-in palettes**: Comes with popular colorschemes (Nord, Gruvbox, Catppuccin)
- **Custom Palettes**: Load in your favorite custom palettes.

---
# Installation
### Installing from source
Clone the repository and install it:

```bash
git clone https://github.com/Phinixprono123/theemer-rs.git
cd theemer-rs
cargo install --path .
```
---
## Usage

```bash
theemer-rs <INPUT_FILE> [OPTIONS]
```

By default it uses the nord theme and 'output.jpg' for output file.
```bash
theemer-rs input.png
```
Specify output file name
```bash 
theemer-rs input.png -o output.png
```

Specify the built-in theme  
```bash
theemer-rs input.png --theme gruvbox
```
Available built-in themes: `nord`, `gruvbox`, `catppuccin`

Use a custom theme palette file 
```bash
theemer-rs input.png --theme-file palette.txt
```
*Note: Custom theme files should contain 6-digit hex color codes separated by spaces or newlines (e.g., #1e1e2e #a6e3a1 #f38ba8).*

Adjust temparature value (color blending smoothness)
```bash
theemer-rs input.png --temp 6.0
```

# Examples

### Example 1 (Catppuccin)

<table>
  <tr>
    <th width="50%" align="center">Original</th>
    <th width="50%" align="center">Themed</th>
  </tr>
  <tr>
    <td><img src="assets/example1o.jpg" alt="Example 1 Original"></td>
    <td><img src="assets/example1t.jpg" alt="Example 1 Themed"></td>
  </tr>
</table>

### Example 2 (Nord)

<table>
  <tr>
    <th width="50%" align="center">Original</th>
    <th width="50%" align="center">Themed</th>
  </tr>
  <tr>
    <td><img src="assets/example2o.jpg" alt="Example 2 Original"></td>
    <td><img src="assets/example2t.jpg" alt="Example 2 Themed"></td>
  </tr>
</table>

### Example 3 (Gruvbox)

<table>
  <tr>
    <th width="50%" align="center">Original</th>
    <th width="50%" align="center">Themed</th>
  </tr>
  <tr>
    <td><img src="assets/example3o.jpg" alt="Example 3 Original"></td>
    <td><img src="assets/example3t.jpg" alt="Example 3 Themed"></td>
  </tr>
</table>


