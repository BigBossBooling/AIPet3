# Financial Document Generator

This project aims to provide a system for generating various financial documents such as checks, bank statements, W-2 forms, etc.

## Project Structure

- `app/`: Contains the core application logic, including document generation modules.
- `templates/`: Holds HTML templates for the different financial documents.
- `static/`: Stores static files like CSS and JavaScript.
- `tests/`: Contains unit tests for the project.
- `requirements.txt`: Lists the Python dependencies for the project.

## Setup and Installation

1.  **Clone the repository (if applicable):**
    ```bash
    # git clone <repository_url>
    # cd financial-document-generator
    ```

2.  **Create a virtual environment (recommended):**
    ```bash
    python -m venv venv
    source venv/bin/activate  # On Windows use `venv\Scripts\activate`
    ```

3.  **Install dependencies:**
    ```bash
    pip install -r requirements.txt
    ```

## Running the Application

(Instructions on how to run the application will be added here as development progresses.)

## Running Tests

To run the tests, navigate to the project's root directory (`financial_document_generator/`) and run:

```bash
python -m unittest discover tests
```

Or, to run a specific test file:
```bash
python -m unittest tests.test_check_generator
```
(Replace `test_check_generator` with the desired test file)

## Contributing

(Details on how to contribute to the project will be added here.)
