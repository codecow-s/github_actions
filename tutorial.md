# github actions 教程
## 1. 什么是 Github Actions
Github Actions 是 Github 提供的一项持续集成服务，它可以让我们在代码仓库中自动化地执行一些任务，比如测试、构建、部署等。它的核心是一个事件驱动的任务执行引擎，可以根据事件触发任务的执行。比如，当我们的代码仓库中有新的代码提交时，Github Actions 可以自动运行测试，当我们的代码仓库中有新的 tag 时，Github Actions 可以自动构建并部署我们的应用。

## 2. Github Actions 的工作流程
Github Actions 的工作流程是这样的：当代码仓库中的某个事件发生时，Github Actions 会根据我们事先定义好的工作流文件（.github/workflows/*.yml）来执行相应的任务。工作流文件中定义了一系列的任务，每个任务都是一个独立的步骤，可以是一个 shell 命令、一个脚本、一个 Docker 容器等。每个任务都可以依赖于其它任务，可以并行执行，也可以按照顺序执行。

## 3. Github Actions 的工作流文件
Github Actions 的工作流文件是一个 YAML 格式的文件，它可以定义一个或多个工作流。每个工作流都是一个独立的任务流程，可以根据需要定义多个工作流。工作流文件的命名规则是：.github/workflows/*.yml。比如，我们可以在代码仓库中创建一个 .github/workflows/test.yml 文件来定义一个测试任务的工作流。
