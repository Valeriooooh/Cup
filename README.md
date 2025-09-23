# ☕ Cup - Your Perfect Java/Kotlin Build System

*Because every good developer needs their daily Cup of Java ☕*

[![Made with Love](https://img.shields.io/badge/Made%20with-❤️-red.svg)](https://github.com/Valeriooooh/Cup)
[![Powered by Coffee](https://img.shields.io/badge/Powered%20by-☕-brown.svg)](https://github.com/Valeriooooh/Cup)
[![Language Support](https://img.shields.io/badge/Languages-Java%20%26%20Kotlin-orange.svg)](https://github.com/Valeriooooh/Cup)

## 🤔 What is Cup?

Cup is a blazingly fast ⚡ (okay, maybe just reasonably fast) build system for Java and Kotlin projects. Named after the most important ingredient in any developer's life - coffee ☕ - Cup aims to make your Java/Kotlin development experience as smooth as a perfectly brewed espresso.

**Why "Cup"?** Because:
- Java ☕ → Coffee ☕ → Cup ☕
- It's short and sweet (like a good espresso shot)
- You can `cup build` and `cup run` just like you can "cup your hands" 🙌
- It holds all your code together (like a cup holds coffee)

## 🚀 Features That'll Make You Go "Woohooo!" 

- 🔥 **Mixed Java/Kotlin Support**: Write Java, Kotlin, or mix them like a caffeinated cocktail
- 🎯 **Simple Commands**: `cup new`, `cup build`, `cup run`, `cup doc` - that's it!
- 📁 **Smart Project Structure**: Follows Maven-style conventions (because we're not savages)
- 🔧 **Zero Config**: Works out of the box, configurable when you need it
- 📚 **Auto Documentation**: Generate docs faster than you can say "JavaDoc"
- 🏃‍♂️ **Lightning Fast**: Compiles your code while you're still reaching for your coffee mug
- 🎨 **Pretty Output**: Because ugly terminal output is for people who drink instant coffee

## 🛠️ Installation

```bash
# Clone this beauty
git clone https://github.com/Valeriooooh/cup
cd cup

# Build it (you'll need Rust - the language that's faster than your morning coffee routine)
cargo build --release

# Add it to your PATH and start brewing! ☕
```

## 🏃‍♂️ Quick Start - From Zero to Hero in 30 Seconds

### Create a new Java project (Classic ☕)
```bash
cup new my-awesome-app
cd my-awesome-app
cup run
# Output: Hello, World! 🌍
```

### Create a new Kotlin project (Modern ☕)
```bash
cup new my-kotlin-masterpiece --kotlin
cd my-kotlin-masterpiece
cup run
# Output: Hello, World! 🌍 (but with more syntactic sugar)
```

### Mixed project (The best of both worlds ☕☕)
```bash
cup new my-polyglot-adventure --kotlin
cd my-polyglot-adventure
# Add some Java files in src/main/java/
# Add some Kotlin files in src/main/kotlin/
cup build  # Watch the magic happen ✨
cup run    # Profit! 💰
```

## 📋 Commands That Actually Make Sense

| Command | What it does | Coffee analogy |
|---------|-------------|----------------|
| `cup new <name>` | Creates a new project | Ordering a fresh cup ☕ |
| `cup new <name> --kotlin` | Creates a Kotlin project | Ordering a fancy latte with extra foam ☕✨ |
| `cup build` | Compiles your masterpiece | Grinding those beans 🫘 |
| `cup run` | Runs your application | Taking that first perfect sip ☕😌 |
| `cup doc` | Generates documentation | Reading the coffee menu 📖☕ |

## 📁 Project Structure (Or "How We Organize Our Coffee Beans")

```
your-awesome-project/
├── Cup.toml              # The recipe for your coffee ☕
├── src/
│   ├── main/
│   │   ├── java/         # Your Java beans 🫘
│   │   └── kotlin/       # Your Kotlin beans (premium blend) 🫘✨
│   └── test/
│       ├── java/         # Java tests (quality control) ✅
│       └── kotlin/       # Kotlin tests (fancy quality control) ✅✨
├── lib/                  # External JARs (imported coffee) 🫙
├── build/                # Compiled goodness 🏗️
└── doc/                  # Documentation (the manual) 📚
```

## ⚙️ Configuration (Cup.toml) - Your Coffee Recipe

```toml
[project]
name = "my-caffeinated-project"
version = "1.0.0"
main_class = "main.MainKt"  # For Kotlin: MainKt, for Java: Main

[build]
source_dir = "src/main"            # Where the magic happens ✨
output_dir = "build/classes"       # Where the compiled magic goes 🎯
test_dir = "src/test"              # Where we make sure it works ✅
java_version = "11"                # Because we're not living in 2006 📅
doc_dir = "doc"                    # Where we document our genius 🧠

[dependencies]
# Add your JAR dependencies here
# somelib = "path/to/somelib.jar"
```

## 🎭 Java ❤️ Kotlin Love Story

Cup handles mixed Java/Kotlin projects like a matchmaker:

**Java class (the classic one):**
```java
// src/main/java/util/Helper.java
package util;

public class Helper {
    public String greet(String name) {
        return "Hello, " + name + "! ☕";
    }
}
```

**Kotlin file (the cool kid):**
```kotlin
// src/main/kotlin/Main.kt
package main

import util.Helper

fun main() {
    val helper = Helper()
    println(helper.greet("Coffee Lover")) // Hello, Coffee Lover! ☕
    
    // Look ma, Java interop! 🎉
}
```

**Cup's magic compilation process:**
1. 🔮 Compiles Kotlin files (including Java source references)
2. ⚡ Compiles Java files (with Kotlin classes in classpath)
3. 🎉 Everything works together harmoniously

## 🤯 Advanced Usage (For Coffee Connoisseurs)

### Custom Java Version
```bash
# In your Cup.toml
java_version = "17"  # Living in the future! 🚀
```

### External Dependencies
```bash
# Drop your JARs in the lib/ directory
cp awesome-library.jar lib/
cup build  # Cup automatically includes them ✨
```

### Documentation Generation
```bash
cup doc
# Opens doc/index.html in your heart (and browser) 💖
```

## 🐛 Troubleshooting (When Your Coffee Gets Cold)

### "kotlinc not found" 😱
```bash
# Install Kotlin (the right way)
# Ubuntu/Debian: sudo snap install kotlin --classic
# macOS: brew install kotlin  
# Windows: *confused screaming* (just download from kotlinlang.org)
```

### "My mixed compilation failed!" 😭
```bash
# Check your versions
kotlinc -version
javac -version

# Make sure both exist and are recent-ish
```

### "It compiles but doesn't run!" 🤬
```bash
# Check your main class name:
# Java: "com.example.Main"
# Kotlin: "com.example.MainKt" (note the Kt suffix)
```

### "I'm getting weird classpath errors!" 🙃
```bash
# Clear your build directory and try again
rm -rf build/
cup build
# If that doesn't work, make more coffee and try again ☕
```

## 🤝 Contributing (Join the Coffee Circle)

We welcome contributions! Whether you:
- 🐛 Found a bug (and didn't just blame it on insufficient coffee)
- 💡 Have a feature idea (preferably coffee-related)
- 📝 Want to improve documentation (make it funnier!)
- ☕ Just want to discuss coffee preferences

Open an issue or PR! But first, make sure you've had your coffee ☕

### Development Setup
```bash
git clone https://github.com/yourusername/cup
cd cup
cargo build
# Grab a coffee ☕
```

<!-- ## 🎯 Roadmap (Our Coffee Dreams) -->

<!-- - [ ] 🔌 Plugin system (for when you want to add your own flavor) -->
<!-- - [ ] 📦 Package management (because managing dependencies is like managing coffee suppliers) -->
<!-- - [ ] 🧪 Better testing support (unit tests with a side of espresso) -->
<!-- - [ ] 🌐 Web framework integration (serve your Java with a side of HTTP) -->
<!-- - [ ] 🐳 Docker support (containerized coffee, anyone?) -->
<!-- - [ ] 🏆 IDE integration (because terminals are so last century) -->
<!-- - [ ] ☁️ Cloud deployment (take your coffee to the cloud!) -->

## 🙋‍♀️ FAQ (Frequently Asked Coffee Questions)

**Q: Why not just use Maven/Gradle?**  
A: Have you *tried* configuring Maven? It's like trying to make coffee with a manual that's written in ancient Latin. Cup just works. ☕

**Q: Is this production ready?**  
A: Define "production"... If by production you mean "produces coffee-fueled applications," then absolutely! ☕✅

**Q: What if I prefer tea?**  
A: We don't judge, but we also don't understand. This tool is definitely coffee-optimized. 🫖❓

**Q: Can I use this for my startup that's going to revolutionize [insert buzzword]?**  
A: Sure! But remember to give us credit when you're sipping champagne in your penthouse. 🥂🏢

**Q: What's the difference between Cup and other build tools?**  
A: Cup has more emojis and coffee references per line of documentation. Also, it actually works without requiring a PhD in XML. 📜☕

## 📜 License

MIT License - because we believe in sharing coffee (and code) freely! ☕📄

Made with ❤️, lots of ☕, and a healthy dose of 🤪 by developers who believe that build tools shouldn't require a computer science degree to understand.

---

**Remember**: Life's too short for bad coffee and complicated build systems. Use Cup! ☕✨

*P.S. - No actual coffee was harmed in the making of this build system. But a lot was consumed. ☕💀*
