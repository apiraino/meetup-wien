Rust has a great strong typing system, but as soon as one starts structuring an application, it becomes apparent how it can easily end up duplicating code. A bit more flexibility is the solution when building the business logic.
What if you could define a shared behavior between different structures? Common Traits come to the rescue, but sometimes zero-cost Traits are not enough, and a minimal runtime is needed.
We will show in simple terms what the problem is about, introducing Object Traits and step by step we will lead the audience to the suggested solution, exploring how the runtime introduced by Object Trait differs from other programming languages and how it cost in terms of performance.

---

A lot of what we do happens on the Web. The browser is nowadays the single tool that runs many applications. However, stand-alone, desktop-centric applications are far from being forgotten. Would you like to see the basics of writing a GUI application in Rust?
GTK-rs is the most promising GUI toolkit. In this short introduction we will show how to build a simple GUI that uses standard widgets, just to whet your appetite. Bonus point, how to run async tasks without blocking the main GUI.

[Explain Trait Object](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
