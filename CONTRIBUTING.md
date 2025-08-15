# Contributing to r2048

Thank you for considering contributing to r2048! Any contributions that help improve this Rust implementation of the classic 2048 game are welcome. Here are some guidelines to help you get started:

## How to Contribute

1. **Fork the Repository**

   Create a fork of the repository by clicking the "Fork" button at the top of the GitHub page.

2. **Clone Your Fork**

   Clone your forked repository to your local machine:
   ```bash
   git clone https://github.com/YOUR_USERNAME/r2048.git
   cd r2048
   ```

3. **Create a Branch**

   Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature-or-bugfix-name
   ```

4. **Make Changes**

   Make your changes to the codebase. Ensure your code follows Rust best practices and the project's coding standards.

5. **Test Your Changes**

   Run the game manually to ensure gameplay works correctly:
   ```bash
   cargo run
   ```

6. **Format Your Code**

   Ensure your code is properly formatted:
   ```bash
   cargo fmt
   ```

7. **Commit Your Changes**

   Commit your changes with a clear and descriptive commit message:
   ```bash
   git commit -m "Add feature or fix bug description"
   ```

8. **Push Your Changes**

   Push your changes to your forked repository:
   ```bash
   git push origin feature-or-bugfix-name
   ```

9. **Create a Pull Request**

   Open a pull request from your branch to the `main` branch of the original repository. Provide a detailed description of your changes.

## 🐛 Bug Reports

When reporting bugs, please include:

• **Steps to reproduce** - Clear, numbered steps  
• **Expected behavior** - What should happen  
• **Actual behavior** - What actually happens  
• **Environment** - Your OS and Rust version  
• **Logs** - Any error messages or relevant output  

## ✨ Feature Requests

For new features, please:

• **Search first** - Check existing issues to avoid duplicates  
• **Describe clearly** - Explain the feature and its benefits  
• **Consider compatibility** - Think about backward compatibility  
• **Discuss implementation** - Share your ideas on how to build it  

## 📚 Documentation

• Update docs for any new features or API changes  
• Add code comments for complex game logic  
• Update README.md if your changes affect usage  

## 🎯 Game Logic Guidelines

When contributing to game mechanics:

• **Stay compatible** - Maintain original 2048 rules  
• **Be deterministic** - Ensure moves are reproducible  
• **Handle edge cases** - Full board, no valid moves, etc.  
• **Consider performance** - Optimize for larger board sizes  

---

*Let's make this the best Rust implementation of 2048 together!*
