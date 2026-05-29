## 引用

> Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

> When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. 

> When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. 

## 笔记
- `Cargo.lock` 的作用是可重现的工作，在第一次运行 cargo build 后，会将依赖的版本写入 `Cargo.lock` 文件中，以便后续的构建可以使用相同的版本。
- 可以通过 `cargo update` 命令更新依赖的版本，并将最新的版本写入 `Cargo.lock` 文件中。'
- 可以使用 `cargo doc --open` 命令生成文档并在浏览器中打开。
