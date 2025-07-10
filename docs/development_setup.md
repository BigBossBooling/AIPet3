# CritterCraftUniverse: Development Setup Guide

## 1. Overview

This guide provides instructions for setting up your local development environment for CritterCraftUniverse. The project consists of a Python backend (game logic, blockchain interaction) and a React frontend (user interface).

## 2. Prerequisites

* **Git:** For version control.
* **Python 3.9+:** Recommended for the backend.
* **pip:** Python package installer (usually comes with Python).
* **Node.js (LTS recommended):** For the frontend.
* **npm (or yarn):** Node.js package manager (comes with Node.js).
* **Rust (with Cargo):** For Substrate pallet development (blockchain logic).
    * Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    * Add `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
* **Docker & Docker Compose:** For running local blockchain nodes (conceptual).

## 3. Getting Started

**3.1. Clone the Repository:**

```bash
git clone [YOUR_REPO_URL]
cd [YOUR_REPO_DIRECTORY_NAME] # e.g., BlockChain/critter-craft
```

**3.2. Backend Setup (Python):**
* Create and Activate Virtual Environment:
  ```bash
  python3 -m venv venv_backend
  source venv_backend/bin/activate # On Windows: .\venv\Scripts\activate
  ```
* Install Dependencies:
  ```bash
  pip install -r requirements.txt
  ```
* Run Backend Tests:
  ```bash
  pytest -vv
  ```
  * Note: Ensure `pytest.ini` is configured correctly for test discovery.

**3.3. Frontend Setup (React):**
* Navigate to Frontend Directory:
  ```bash
  cd frontend/
  ```
* Install Dependencies:
  ```bash
  npm install
  ```
  * Important Note: This step may encounter sandbox limitations in certain environments (e.g., "affected too many files" error). If so, manual dependency installation or local verification in a full development environment is recommended.
* Run Frontend Tests:
  ```bash
  npm run test
  ```

## 4. Running the Application (Conceptual)

* **Blockchain Node:**
    * Start a local EmPower1 Blockchain node (conceptual instructions).
* **Backend Services:**
    * Start the CritterCraftUniverse Python backend services (conceptual instructions).
* **Frontend Development Server:**
    * ```bash
      cd frontend/ && npm start
      ```

## 5. Code Style & Linting

* **Python:**
    * `black`: `pip install black` (configured via `pyproject.toml`).
    * `flake8`: `pip install flake8` (configured via `.flake8`).
    * Run: `black .` and `flake8 .`
* **TypeScript/JavaScript (Frontend):**
    * ESLint, Prettier: (configured via `.eslintrc.js`, `.prettierrc.js`).
    * Run: `npm run lint` (or equivalent).

## 6. Contribution Guidelines

Please refer to `CONTRIBUTING.md` for detailed guidelines on contributing to CritterCraftUniverse, including:
* Code of Conduct adherence.
* Pull Request process.
* Testing procedures (emphasizing TDD).
* Branching strategy.
