# Contributing to CritterCraftUniverse

We welcome and encourage contributions to CritterCraftUniverse! By contributing, you help us sculpt the future of digital companionship and build a vibrant, decentralized ecosystem. Please adhere to the following guidelines and principles.

## 1. Our Guiding Principles

All contributions must align with the core philosophies of Josephis K. Wade, The Architect:

* **The Expanded KISS Principle:**
    * **Keep it Clear:** Code must be readable, well-commented, and self-documenting. Designs should be transparent.
    * **Iterate Intelligently:** Build in small, verifiable increments. Avoid over-engineering.
    * **Systematize for Scalability:** Design for massive scale from day one.
    * **Synchronize for Synergy:** Ensure components work together seamlessly.
* **Law of Constant Progression:** We are always moving forward, continuously improving and evolving.
* **Precision In, Prowess Out:** Meticulous attention to detail in code leads to robust, high-quality outcomes.
* **Battle-Tested Mentality:** Assume failure and corruption are inevitable; build resilient systems.
* **No Magic:** Solutions must be deterministic and understandable.
* **QRASL Code of Conduct:** All interactions and contributions must adhere to the [QRASL Code of Conduct](../docs/CODE_OF_CONDUCT.md).

## 2. Getting Started

Please refer to the [Development Setup Guide](../docs/development_setup.md) for instructions on setting up your local development environment.

## 3. Contribution Workflow

1.  **Fork the Repository:** Start by forking the main CritterCraftUniverse repository.
2.  **Create a Feature Branch:** For each new feature or bug fix, create a new branch from `main` (e.g., `feature/my-new-feature` or `bugfix/fix-login-bug`).
3.  **Develop with Test-Driven Development (TDD):**
    * **Write Tests First:** For any new functionality, write unit and/or integration tests that define the expected behavior *before* writing the implementation code.
    * **Red, Green, Refactor:** See the test fail (Red), write code to make it pass (Green), then refactor for clarity and efficiency.
4.  **Adhere to Code Style & Linting:**
    * Before committing, ensure your code adheres to our style guidelines.
    * **Python:** Run `black .` and `flake8 .` (configured via `pyproject.toml` and `.flake8`).
    * **JavaScript/TypeScript:** Run `npm run lint` and `npm run format` (or equivalent, configured via `.eslintrc.js` and `.prettierrc.js`).
5.  **Run All Tests:** Before submitting a Pull Request, ensure all existing backend and frontend tests pass successfully.
    * **Backend:** `pytest -vv` (from `BlockChain/critter-craft` or `BlockChain/`).
    * **Frontend:** `npm run test` (from `frontend/`).
    * **Note:** Be aware of potential sandbox limitations for frontend tests and verify locally if needed.
6.  **Write Clear Commit Messages:** Use concise and descriptive commit messages following conventional commits (e.g., `feat: Add new pet interaction`, `fix: Resolve login bug`).

## 4. Pull Request (PR) Process

1.  **Open a Pull Request:** Submit your changes via a Pull Request to the `main` branch.
2.  **Describe Your Changes:** Clearly explain what problem your PR solves, how it solves it, and any relevant technical details.
3.  **Link to Issues:** Reference any related issues (e.g., `Closes #123`).
4.  **Code Review:** Your PR will be reviewed by maintainers. Be open to feedback and iterative improvements.

## 5. Code of Conduct

All participants are expected to adhere to the [QRASL Code of Conduct](../docs/CODE_OF_CONDUCT.md). Violations will not be tolerated.

## 6. Reporting Issues

If you find a bug or have a feature request, please open an issue on our GitHub repository.

---
*Thank you for contributing to CritterCraftUniverse! Together, we will sculpt the future of digital companionship.*
