<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Beta-blue.svg" /> 
</p>

<h1 align="center">🏋️‍♂️ Rusty Gym Membership System</h1>

<p align="center">
  A streamlined Command Line Interface (CLI) designed to handle member registration and financial analytics for small to medium fitness centers.
</p>

---

## 🎓 Educational Disclaimer
This project is a milestone in my **Personal Apprenticeship** with Rust. 
* **Focus**: Mastering data collections (`Vec`), arithmetic casting, and terminal-based user interaction.
* **Goal**: Implementing clean data structures to manage relational information effectively.

## 🌟 Features
* **Batch Registration**: Input any number of members in a single session.
* **Smart Analytics**: 
    * Automated total revenue calculation.
    * Real-time member age averaging.
* **Memory Safety**: Uses Rust's ownership model to manage strings and structs without leaks.
* **Detailed Debugging**: Full inventory list display at the end of each session.

## 🛠️ Technical Deep Dive
* **Type Casting**: Handles conversions between `usize` (input), `u32` (age), and `f32` (averages/prices) with precision.
* **Iterators & Closures**: Uses functional patterns like `.map()` and `.sum()` to process member data efficiently.
* **Struct Hierarchy**: Implements an `Object-oriented` style approach using `Member` structs and `SubscriptionType` enums.



---

## 🚀 How to Run
1. Clone the repository.
2. Run the application:
   ```bash
   cargo run

## ⚖️ License & Copyright

Copyright (c) 2026 **[dandiest]**

This project is licensed under the MIT License.

You are free to use, study, and modify this code for educational purposes. Feel free to fork it if you are on your own Rust learning journey!
