# Financial Document Generator (AIPet3)

This project is a Flask-based web application for generating various financial documents programmatically and via a web interface.

## Overview

(To be filled in - e.g., description of supported documents, technologies used)

## Features

*   Web forms for generating:
    *   W-2 Forms
    *   Checks
    *   Check Stubs / Earning Statements
    *   Bank Statements
    *   Income Statements
*   JSON API endpoints for programmatic generation of:
    *   W-2 Forms
    *   Checks
*   PDF output for all documents.
*   Utilizes WeasyPrint for HTML to PDF conversion.

## Project Structure

(To be filled in - brief overview of key directories like `app`, `templates`, `static`, `generators`, `tests`)

## Setup and Installation

1.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    cd financial-document-generator
    ```
2.  **Create a virtual environment (recommended):**
    ```bash
    python3 -m venv venv
    source venv/bin/activate  # On Windows use `venv\Scripts\activate`
    ```
3.  **Install dependencies:**
    ```bash
    pip install -r financial_document_generator/requirements.txt
    ```
    (Note: On some systems, you might need to install Pango, Cairo, and GDK-PixBuf for WeasyPrint. Refer to WeasyPrint documentation for platform-specific instructions.)

## Running the Application (Development)

The Flask application is defined in `financial_document_generator/app/main.py`.

To run the development server:
```bash
python financial_document_generator/app/main.py
```
The application will typically be available at `http://127.0.0.1:5000/` or `http://0.0.0.0:5000/`.

## Web Interface

Navigate to the application's base URL (e.g., `http://localhost:5000/`) in your web browser. You will find links or forms to generate the supported documents:
*   `/w2`: W-2 Form data entry.
*   `/check`: Check data entry.
*   `/check-stub`: Check Stub data entry.
*   `/earning-statement`: Earning Statement data entry.
*   `/bank-statement`: Bank Statement data entry.
*   `/income-statement`: Income Statement data entry.

## Running Tests

(To be filled in - instructions on how to run `unittest` tests, e.g., `python -m unittest discover -s financial_document_generator/tests -p 'test_*.py'`)

## API Endpoints

The application provides API endpoints for programmatic generation of financial documents. All API endpoints expect JSON in the request body and return a PDF file on success or a JSON error message on failure.

### Generate W-2 Form

*   **URL:** `/api/v1/w2/generate`
*   **Method:** `POST`
*   **Description:** Generates a W-2 PDF document.
*   **Request Body (JSON):**
    *   **Required String Fields:**
        *   `employee_name`
        *   `employee_ssn`
        *   `employer_name`
        *   `employer_ein`
    *   **Required Numeric Fields (can be string or number, will be converted to float):**
        *   `wages_tips_other_compensation`
        *   `federal_income_tax_withheld`
    *   **Optional String Fields (default to empty string if not provided):**
        *   `state_employer_state_id_no`
        *   `locality_name`
        *   `employee_address` (Note: While optional in API validation, typically needed for a complete W-2)
        *   `employee_city_state_zip` (Note: Similar to above)
        *   `employer_address` (Note: Similar to above)
        *   `employer_city_state_zip` (Note: Similar to above)
        *   `control_number`
        *   `other_description_code_d`
    *   **Optional Numeric Fields (default to 0.0 if not provided):**
        *   `social_security_wages`
        *   `medicare_wages_and_tips`
        *   `social_security_tax_withheld`
        *   `medicare_tax_withheld`
        *   `social_security_tips`
        *   `allocated_tips`
        *   `dependent_care_benefits`
        *   `nonqualified_plans`
        *   `statutory_employee` (0 or 1)
        *   `retirement_plan` (0 or 1)
        *   `third_party_sick_pay` (0 or 1)
        *   `other_amount_code_d`
        *   `state_wages_tips_etc`
        *   `state_income_tax`
        *   `local_wages_tips_etc`
        *   `local_income_tax`
*   **Example Request:**
    ```json
    {
        "employee_name": "John Doe",
        "employee_ssn": "000-00-0000",
        "employee_address": "123 Main St",
        "employee_city_state_zip": "Anytown, ST 12345",
        "employer_name": "Acme Corp",
        "employer_address": "456 Business Rd",
        "employer_city_state_zip": "Businesstown, BS 67890",
        "employer_ein": "12-3456789",
        "control_number": "CTRL123",
        "wages_tips_other_compensation": 60000.00,
        "federal_income_tax_withheld": 7000.00,
        "social_security_wages": 60000.00,
        "medicare_wages_and_tips": 60000.00,
        "social_security_tax_withheld": 3720.00,
        "medicare_tax_withheld": 870.00,
        "state_employer_state_id_no": "CA-12345",
        "state_wages_tips_etc": 60000.00,
        "state_income_tax": 2000.00
    }
    ```
*   **Success Response:**
    *   Status Code: `200 OK`
    *   Body: PDF file content.
*   **Error Responses:**
    *   Status Code: `400 Bad Request` (e.g., missing fields, invalid data types) - Body: `{"error": "description"}`
    *   Status Code: `415 Unsupported Media Type` (if request is not JSON) - Body: `{"error": "Request must be JSON"}`
    *   Status Code: `500 Internal Server Error` (if PDF generation fails) - Body: (HTML error page or JSON `{"error": "description"}` if API specific 500 handler is set up)

### Generate Check

*   **URL:** `/api/v1/check/generate`
*   **Method:** `POST`
*   **Description:** Generates a Check PDF document.
*   **Request Body (JSON):**
    *   **Required String Fields:**
        *   `bank_name`
        *   `check_number`
        *   `payee_name`
        *   `amount_words` (e.g., "ONE HUNDRED AND 00/100")
        *   `routing_number`
        *   `account_number`
    *   **Required `amount_numeric` Field:**
        *   Can be string (e.g., "100.00") or number (e.g., 100.00). Will be formatted to a string with two decimal places.
    *   **Optional String Fields:**
        *   `bank_address`
        *   `memo`
        *   `bank_logo_url` (URL or local path accessible by server - if local, path should be relative to project root or absolute)
        *   `date_output_format` (e.g., "%Y-%m-%d", if provided, `date` should be a valid date string interpretable by this format if CheckGenerator uses it explicitly, otherwise generator uses its default for date objects)
    *   **Optional Date Field:**
        *   `date` (string, 'YYYY-MM-DD' format, e.g., "2023-12-25")
*   **Example Request:**
    ```json
    {
        "bank_name": "Community Bank",
        "check_number": "1001",
        "date": "2023-12-25",
        "payee_name": "Jane Smith",
        "amount_numeric": 150.75,
        "amount_words": "ONE HUNDRED FIFTY AND 75/100",
        "memo": "Invoice #INV123",
        "routing_number": "123456789",
        "account_number": "987654321"
    }
    ```
*   **Success Response:**
    *   Status Code: `200 OK`
    *   Body: PDF file content.
*   **Error Responses:** (Similar to W2 endpoint)

## Contributing

(To be filled in - guidelines for contributing, code style, pull requests)

## License

(To be filled in - e.g., MIT License. Add a `LICENSE` file if one doesn't exist.)