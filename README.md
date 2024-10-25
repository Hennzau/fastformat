# 🚀 **fastformat: High-Performance Data Processing Library**

[![Build Status](https://img.shields.io/github/workflow/status/dora-rs/fastformat/CI)](https://github.com/dora-rs/fastformat/actions)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/github/v/tag/dora-rs/fastformat)](https://github.com/dora-rs/fastformat/tags)

## 🎯 **Project Objective**

The goal of **fastformat** is to build an **efficient**, **real-time** data processing library that supports formats like **NDarray**, **Numpy**, and **Arrow**, without unnecessary data copies. ⚡

This independent library enables **simple and fast** data conversion between formats, ensuring optimal performance across various platforms.

🌟 Key features of **fastformat**:
- **💼 Independent Library**: Usable with or without [`dora`](https://github.com/dora-rs). Find the repo [here](https://github.com/dora-rs/fastformat).
- **🌐 Agnostic Format**: The library is designed to support various data formats like **Numpy**, **Arrow**, and others, with conversion through `into_[format]` functions.
- **🦀 Rust & 🐍 Python Integration**: The core is implemented in **Rust** for speed and portability, with a Python interface using **PyO3** for ease of use and compatibility.
- **📦 Minimal Dependencies**: Built with **Rust**, fastformat ensures minimal external dependencies and maximum cross-platform compatibility.
- **🔄 Simplicity in Conversion**: fastformat doesn’t aim to handle complex data operations on its own. Instead, it provides a simple interface to wrap and convert data types efficiently, leaving complex operations to other specialized projects.

> **Note**: fastformat is **not** designed to be a fully-featured API for performing advanced operations on specific data types. Instead, it focuses on providing **simple interfaces** for handling data representations in various formats.

---

## 💻 **Technology Stack**

- **Rust** 🦀 for core functionality and high-performance processing.
- **PyO3** 🐍 for seamless integration with Python.
- **Arrow** 🏹 for powerful in-memory data representation.
- **Kornia-rs** 🖼️ as an **OpenCV replacement** in Rust for advanced image processing when needed.

---

## 🚧 **Installation Instructions**

### Rust

```Cargo.toml
[dependencies]
fastformat = { version = "0.1.0" }
```

### Python

Coming soon!

---

## 📚 **Usage Example**

Here’s a simple example of how to use **fastformat** to convert data formats:

### Rust

```rust
// Create a 100% plain rust struct
pub struct CustomDataType {
    size: u32,
    label: String,
    ranges: Vec<u8>,
}

// Add this trait to the struct to enable conversion to Arrow format
impl IntoArrow for CustomDataType {
    fn into_arrow(self) -> eyre::Result<ArrowArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<UInt32Type>("size", self.size)
            .push_utf8_singleton("label", self.label)
            .push_primitive_array::<UInt8Type>("ranges", self.ranges);

        builder.build()
    }

    fn from_arrow(array_data: ArrowArrayData) -> eyre::Result<Self> {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let size = consumer.primitive_singleton::<UInt32Type>("size")?;
        let label = consumer.utf8_singleton("label")?;
        let ranges = consumer.primitive_array::<UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges,
        })
    }
}

fn main() -> eyre::Result<()> {
    let custom_data = CustomDataType {
        size: 42,
        label: "Hello, World!".to_string(),
        ranges: vec![1, 2, 3, 4, 5],
    };

    // Consume the custom data and convert it into Arrow data
    let arrow_data = custom_data.into_arrow()?;

    // Convert the Arrow data back into custom data
    let custom_data = CustomDataType::from_arrow(arrow_data)?;

    Ok(())
}
```

### Python

```python
@dataclass
class CustomDataType:
    size: np.uint32
    label: str
    ranges: np.ndarray

    def into_arrow(self) -> pa.UnionArray:
        return into_arrow(
            children=[
                pa.array([self.size]),
                pa.array([self.label]),
                pa.array(self.ranges)
            ],
            field_names=['size', 'label', 'ranges'])

    @staticmethod
    def from_arrow(data: pa.UnionArray):
        viewer = ArrowViewer(data)

        return CustomDataType(
            size=viewer.primitive_singleton('size'),
            label=viewer.utf8_singleton('label'),
            ranges=viewer.primitive_array('ranges')
        )

custom_data = CustomDataType(
    size=np.uint32(42),
    label='custom',
    ranges=np.array([1, 2, 3], dtype=np.uint32)
)

arrow_data = custom_data.into_arrow()
reconstructed_data = CustomDataType.from_arrow(arrow_data)
```

---

## 📦 **Future Plans**

- GPU Support for faster processing with **CUDA** ⚡
- Extend support to more formats (e.g. **Torch Tensors**, **Pandas DataFrames**).
- Add support for **multithreading** and **distributed computing**.

---

## 🙌 **Contributing**

We welcome contributions! Feel free to submit issues or pull requests. Check the [CONTRIBUTING](./CONTRIBUTING.md) guide for more information.

---

## 📜 **License**

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

---

🚀 Happy coding with **fastformat**!
