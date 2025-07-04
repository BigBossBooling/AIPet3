# Contributing to CritterCraft Universe

Thank you for considering contributing to CritterCraft Universe! This document outlines the process for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please read it before contributing.

## How Can I Contribute?

### Reporting Bugs

- Check if the bug has already been reported in the Issues section
- Use the bug report template to create a new issue
- Include detailed steps to reproduce the bug
- Include screenshots if applicable
- Describe the expected behavior and what actually happened

### Suggesting Features

- Check if the feature has already been suggested in the Issues section
- Use the feature request template to create a new issue
- Describe the feature in detail and why it would be valuable
- Include mockups or diagrams if applicable

### Pull Requests

1. Fork the repository
2. Create a new branch for your feature or bugfix
3. Make your changes
4. Run tests to ensure your changes don't break existing functionality
5. Submit a pull request

## Development Setup

### Backend

```bash
# Clone the repository
git clone https://github.com/your-username/crittercraft-universe.git
cd crittercraft-universe

# Set up Python environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r blockchain_core/critter-craft/requirements.txt

# Run tests
pytest
```

### Frontend

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Run tests
npm run test
```

## Coding Standards

### Python

- Follow PEP 8 style guide
- Use type hints
- Write docstrings for all functions, classes, and modules
- Use meaningful variable and function names

### TypeScript/JavaScript

- Follow the ESLint configuration
- Use TypeScript for all new code
- Write JSDoc comments for functions and components
- Use functional components with hooks for React

## Testing

- Write unit tests for all new features
- Ensure all tests pass before submitting a pull request
- Aim for high test coverage

## Documentation

- Update documentation for any changes to the API
- Document new features thoroughly
- Keep the README up to date

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests after the first line

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the documentation with details of changes if applicable
3. The PR should work on all supported platforms
4. The PR will be merged once it receives approval from maintainers

Thank you for contributing to CritterCraft Universe!