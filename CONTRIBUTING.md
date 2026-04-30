# Contributing to GMFeather 🪶

Thank you for your interest in contributing to GMFeather! To maintain high code quality and architectural integrity, we follow strict development guidelines.

## 📜 Coding Standards

All contributions must adhere to the following principles:

### Architecture & Design
- **Single Responsibility Principle (SRP)**: Each function should perform one specific business task. Keep functions concise (ideally under 20 lines).
- **Layered Separation**: Maintain a strict separation between `Domain` (business rules), `Use Cases` (actions), and `Infrastructure` (technical implementations/adapters).
- **Dependency Injection**: Always inject external dependencies via interfaces or ports to ensure testability and decoupling.
- **Screaming Architecture**: Organize the codebase by business features first, then by technical layers (e.g., `src/modules/compression/domain/`).

### Code Quality
- **Naming**: All code (variables, functions, classes, files) **must** be named in English using explicit action verbs.
- **Self-Documenting Code**: Do **not** write comments. The code logic should be clear enough to be understood without them.
- **Strict Typing**: All variables, parameters, and return types must be explicitly typed.
- **Immutability**: Use immutability by default. Mutation is only allowed within local scopes for performance or readability.

### Robustness & Security
- **Fail-Fast**: Implement immediate error checking. Throw specific business exceptions instead of generic errors.
- **Observability**: Use a structured logger with appropriate levels (Info, Error, Debug). Avoid standard `print` or `console.log` statements.
- **Security**: Never hardcode configuration values or secrets. Use environment variables or secure configuration managers.

## 🌿 Git Workflow

### Branching
- `main`: Production-ready code.
- `develop`: Integration branch for features.
- Feature branches: `feat/feature-name`, `fix/bug-name`, `chore/task-name`.

### Commit Messages
We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc.)
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools and libraries

## 🛠️ Development Process

1. Clone the repository and create your branch from `develop`.
2. Ensure your code passes all linting and type checks.
3. Write unit tests for any new business logic in the `core`.
4. Submit a Pull Request with a clear description of the changes.
