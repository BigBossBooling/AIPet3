# CritterCraft Universe

CritterCraft Universe is an advanced AI-powered blockchain pet simulation platform that creates meaningful digital companions with evolving personalities.

## Key Features

- **Intelligent Companions**: AI-powered pets that develop unique personalities
- **Dynamic NFTs**: Continuously evolving digital assets that reflect real experiences
- **Custom Blockchain**: Purpose-built infrastructure optimized for pet simulation mechanics
- **Emotional Engagement**: Focus on meaningful relationships rather than purely transactional gameplay

## Technology Stack

### Backend
- Python 3.11
- Flask/FastAPI
- SQLAlchemy
- TensorFlow/PyTorch
- LangChain
- Substrate/Polkadot

### Frontend
- React 18
- TypeScript
- Vite
- Three.js
- Polkadot.js API
- Material UI

### Infrastructure
- Docker
- PostgreSQL
- Redis
- Nginx
- GitHub Actions

## Getting Started

### Prerequisites
- Docker and Docker Compose
- Node.js 18+
- Python 3.11+
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/your-org/crittercraft-universe.git
cd crittercraft-universe
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Start the application with Docker:
```bash
docker-compose up -d
```

4. Or run the services individually:

**Backend:**
```bash
cd blockchain_core/critter-craft
pip install -r requirements.txt
python main.py
```

**Frontend:**
```bash
cd frontend
npm install
npm run start
```

## Development

### Backend Development
```bash
# Run tests
cd blockchain_core/critter-craft
pytest

# Run with hot reload
uvicorn main:app --reload
```

### Frontend Development
```bash
cd frontend
npm run dev

# Run tests
npm run test
```

## Deployment

The application can be deployed using Docker Compose:

```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Project Status & Cleanup Efforts

This codebase has undergone an initial cleanup and refactoring pass. Key actions included:
-   Removal of duplicated files and directories.
-   Consolidation of frontend source files and API client.
-   Deletion of redundant root-level Python scripts and patch/bundle artifacts.
-   Setup of `.gitignore` to exclude common unnecessary files (e.g., `node_modules/`, `__pycache__/`, build artifacts).
-   Initial configuration for Python linting/formatting (Black, Flake8) and frontend linting/formatting (ESLint, Prettier) has been added. Developers should run these tools across the codebase to ensure consistency.

**Important Note on `BlockChain/pet/ai/` Module:**
Due to persistent inconsistencies with the development environment's file system tools, the planned consolidation of the Pet AI logic within the `BlockChain/pet/ai/` directory could not be reliably completed. While many redundant files from older/duplicated AI module paths were removed, the final state of `BlockChain/pet/ai/` (including its `config.py`, `README.md`, `pet_core.py`, `main.py`, `__init__.py`, and other associated files) is uncertain and **requires manual verification, consolidation, and correction by the development team.**

Please refer to `PROJECT_BLUEPRINT.md` for a high-level overview of the project's conceptual documents and overall vision.

## Recommendations for Future Work

### 1. Pet AI Module Completion
-   **Verify and Consolidate `BlockChain/pet/ai/`:** Manually review, consolidate, and correct the files and structure within this directory to ensure all necessary Pet AI logic, configuration, and data files are present, correctly structured, and free of duplication.
-   **Update Import Paths:** After consolidation, ensure all Python import paths referencing the Pet AI module are correct throughout the codebase.

### 2. Testing
-   **Increase Test Coverage:** Write more comprehensive unit and integration tests for all critical modules, especially for blockchain pallets, backend logic, and core frontend components.
-   **End-to-End Testing:** Implement end-to-end tests for key user flows (e.g., pet creation, interaction, marketplace transactions, battle sequences).
-   **Test AI Behavior:** Develop strategies for testing the AI-driven pet personalities and behaviors.

### 3. UX/UI Enhancements
-   **Currency Clarity:** Ensure UIs clearly distinguish between $BITS and $AURA for all transactions.
-   **Asset Bridging UX:** Design and implement a highly intuitive and user-friendly interface for asset bridging, providing clear feedback at each step.
-   **Pet Status Display:** Optimize components like `PetStatusCard.jsx` to present comprehensive pet information clearly without overwhelming users.
-   **Marketplace Navigation:** Streamline UI for navigating between Local Marketplace, Global Marketplace, and potential User Shops.
-   **Governance UI:** Develop simple and intuitive interfaces for governance participation (proposals, voting).
-   **User Testing:** Conduct user testing sessions to gather feedback on UI/UX and identify pain points.

### 4. Performance Optimizations
-   **Blockchain Queries:** Implement robust caching strategies for blockchain data frequently accessed by the UI. Profile and optimize slow queries.
-   **AI Computations:** If AI logic for pet personality/behavior proves to be a bottleneck, optimize these computations (e.g., offload to specialized services, simplify models where appropriate).
-   **Frontend Rendering:** Apply React performance best practices, such as virtualization for large lists, memoization, and optimized state management.
-   **Battle System Efficiency:** Ensure battle logic is efficient, especially with multiple participants or complex status interactions.
-   **Profiling & Monitoring:** Implement tools for profiling application performance (backend, frontend, blockchain interactions) and set up monitoring to detect regressions.

### 5. Dependency Management & Security
-   **Regular Audits:** Periodically run `npm audit` (frontend) and `pip check` / `safety check` (Python backend) to identify and remediate vulnerable dependencies.
-   **Security Best Practices:** Continuously review and apply security best practices for input validation, error handling, authentication/authorization, and secrets management.
-   **Formal External Audit:** Strongly consider a formal security audit by a specialized firm before any mainnet launch involving real assets.

### 6. Documentation
-   **Update Developer Docs:** Ensure all developer documentation is updated to reflect changes from the cleanup and any future refactoring.
-   **User Guides:** Create comprehensive user guides for all major features.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Substrate and Polkadot teams for blockchain infrastructure
- OpenAI for AI capabilities
- The open-source community for various libraries and tools