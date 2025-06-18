import unittest
import os
import io # Added for completeness, though not directly used in this initial structure

# Assuming tests are run from the project root, or PYTHONPATH is set up.
# financial_document_generator/
# |-- app/
# |   |-- __init__.py  (from .main import app)
# |   +-- main.py      (app = Flask(...))
# |-- tests/
# |   +-- test_integration.py
# +-- config.py

from financial_document_generator.app import app # Relies on app/__init__.py
from financial_document_generator.config import TestingConfig

class IntegrationTests(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        # Configure the app for testing
        app.config.from_object(TestingConfig)

        # Flask-WTF uses SECRET_KEY from app.config. TestingConfig sets it.
        # If it was only in Config base class and not overridden, manual set might be:
        # if not app.config.get('SECRET_KEY'):
        #    app.config['SECRET_KEY'] = TestingConfig.SECRET_KEY

        # Disable CSRF for easier testing of forms directly
        # For specific CSRF tests, this would be enabled and token managed.
        app.config['WTF_CSRF_ENABLED'] = False
        app.config['SERVER_NAME'] = 'localhost.localdomain' # For url_for to work without request context in some cases

    def setUp(self):
        # Creates a test client for this application
        self.client = app.test_client()
        # self.runner = app.test_cli_runner() # If using Click commands

    def tearDown(self):
        pass # Nothing specific to tear down for each test for now

    # Test for W2 Form Flow
    def test_w2_form_get(self):
        app.logger.debug("Running test_w2_form_get")
        with app.app_context(): # Ensure url_for works if template uses it outside request
            response = self.client.get('/w2')
        self.assertEqual(response.status_code, 200)
        self.assertIn(b"W-2 Form Data Entry", response.data) # Check for a title or unique text from w2_form.html
        self.assertIn(b"Employer Identification Number (EIN)", response.data) # Check for a field label

    def test_w2_form_post_valid_data(self):
        app.logger.debug("Running test_w2_form_post_valid_data")
        # Prepare valid W2 form data (matching W2FormWTF fields)
        form_data = {
            'employer_ein': '12-3456789',
            'employer_name': 'Test Employer Inc.',
            'employer_address': '100 Corp Lane',
            'employer_city_state_zip': 'Biz City, BS 12345',
            'control_number': 'CTRL001', # Optional
            'employee_ssn': '000-00-0001',
            'employee_name': 'Test Employee',
            'employee_address': '1 Test St',
            'employee_city_state_zip': 'Home Town, HT 54321',
            'wages_tips_other_compensation': '50000.50', # WTForms FloatField handles string conversion
            'federal_income_tax_withheld': '5000.25',
            'social_security_wages': '50000.50',
            'medicare_wages_and_tips': '50000.50',
            'social_security_tax_withheld': '3100.03',
            'medicare_tax_withheld': '725.01',
            'social_security_tips': '0.00', # Optional
            'allocated_tips': '0.00', # Optional
            'dependent_care_benefits': '0.00', # Optional
            'nonqualified_plans': '0.00', # Optional
            'box_12a_code': 'D', # Optional
            'box_12a_amount': '1200.00', # Optional
            'box_12b_code': '', # Optional
            'box_12b_amount': '0.00', # Optional
            'statutory_employee': 'y', # BooleanField can take 'y', 'true', 't'
            'retirement_plan': 'true',
            'third_party_sick_pay': '', # Empty string for False
            'other_description_code_d': 'Test Other', # Optional
            'other_amount_code_d': '100.00', # Optional
            'state_employer_state_id_no': 'STID123', # Optional
            'state_wages_tips_etc': '50000.50', # Optional
            'state_income_tax': '1500.00', # Optional
            'local_wages_tips_etc': '0.00', # Optional
            'local_income_tax': '0.00', # Optional
            'locality_name': '', # Optional
        }
        with app.app_context():
            response = self.client.post('/w2/generate', data=form_data, follow_redirects=True)

        self.assertEqual(response.status_code, 200, f"Response data: {response.data.decode('utf-8') if response.data else 'No data'}")
        self.assertEqual(response.mimetype, 'application/pdf')
        self.assertTrue(response.data.startswith(b'%PDF-'))

    def test_w2_form_post_invalid_data_missing_required(self):
        app.logger.debug("Running test_w2_form_post_invalid_data_missing_required")
        form_data = { # Missing 'employer_name' and other required fields
            'employer_ein': '12-3456789',
            'employee_ssn': '000-00-0001',
            # Many required fields are missing
        }
        with app.app_context():
            response = self.client.post('/w2/generate', data=form_data, follow_redirects=True)
        self.assertEqual(response.status_code, 400)
        self.assertIn(b"This field is required.", response.data)
        self.assertIn(b"W-2 Form Data Entry", response.data) # Check form page is re-rendered

    def test_w2_form_post_invalid_data_bad_format(self):
        app.logger.debug("Running test_w2_form_post_invalid_data_bad_format")
        # Data with incorrectly formatted EIN and SSN
        form_data = {
            'employer_ein': '123456789', # Invalid format
            'employer_name': 'Test Employer Inc.',
            'employer_address': '100 Corp Lane',
            'employer_city_state_zip': 'Biz City, BS 12345',
            'employee_ssn': '000000001', # Invalid format
            'employee_name': 'Test Employee',
            'employee_address': '1 Test St',
            'employee_city_state_zip': 'Home Town, HT 54321',
            'wages_tips_other_compensation': '50000.50',
            'federal_income_tax_withheld': '5000.25',
            'social_security_wages': '50000.50',
            'medicare_wages_and_tips': '50000.50',
            'social_security_tax_withheld': '3100.03',
            'medicare_tax_withheld': '725.01',
            # Fill other optional fields to ensure only format validation fails
            'social_security_tips': '0',
            'allocated_tips': '0',
            'dependent_care_benefits': '0',
            'nonqualified_plans': '0',
        }
        with app.app_context():
            response = self.client.post('/w2/generate', data=form_data, follow_redirects=True)
        self.assertEqual(response.status_code, 400)
        self.assertIn(b"EIN must be in XX-XXXXXXX format.", response.data)
        self.assertIn(b"SSN must be in XXX-XX-XXXX format.", response.data)

       # --- Tests for Check API Endpoint ---
       def test_check_api_post_valid_data(self):
           app.logger.debug("Running test_check_api_post_valid_data")
           # Prepare valid JSON data for the Check API
           json_data = {
               "bank_name": "API Test Bank",
               "check_number": "API-1001",
               "date": "2024-01-15", # YYYY-MM-DD format
               "payee_name": "API Recipient",
               "amount_numeric": 250.75, # Can be number or string "250.75"
               "amount_words": "TWO HUNDRED FIFTY AND 75/100",
               "memo": "API Test Memo",
               "routing_number": "012345678",
               "account_number": "987650123"
               # Optional: 'bank_address', 'bank_logo_url', 'date_output_format'
           }
           with app.app_context(): # Ensure URL routing works
                response = self.client.post('/api/v1/check/generate', json=json_data)
           self.assertEqual(response.status_code, 200, f"Response data: {response.data.decode('utf-8') if response.mimetype == 'application/json' else 'PDF Data'}")
           self.assertEqual(response.mimetype, 'application/pdf')
           self.assertTrue(response.data.startswith(b'%PDF-'))

       def test_check_api_post_invalid_json_missing_required(self):
           app.logger.debug("Running test_check_api_post_invalid_json_missing_required")
           json_data = { # Missing 'payee_name', 'amount_numeric', 'amount_words'
               "bank_name": "API Test Bank Fail",
               "check_number": "API-1002",
               "routing_number": "012345678",
               "account_number": "987650123"
           }
           with app.app_context():
                response = self.client.post('/api/v1/check/generate', json=json_data)
           self.assertEqual(response.status_code, 400)
           self.assertEqual(response.mimetype, 'application/json')
           json_response = response.get_json()
           self.assertIn('error', json_response)
           # Check for specific missing field error (example, might list one or all)
           self.assertTrue("Missing required field: payee_name" in json_response['error'] or \
                           "Missing required field: amount_words" in json_response['error'] or \
                           "Missing required field: amount_numeric" in json_response['error'])


       def test_check_api_post_invalid_content_type(self):
           app.logger.debug("Running test_check_api_post_invalid_content_type")
           # Send data as form-data instead of JSON
           form_data = {
               "bank_name": "API Test Bank ContentType",
               "check_number": "API-1003",
               "payee_name": "API Recipient ContentType",
               "amount_numeric": "100.00",
               "amount_words": "ONE HUNDRED AND 00/100",
               "routing_number": "012345678",
               "account_number": "987650123"
           }
           with app.app_context():
                response = self.client.post('/api/v1/check/generate', data=form_data)
           self.assertEqual(response.status_code, 415) # Unsupported Media Type
           self.assertEqual(response.mimetype, 'application/json')
           json_response = response.get_json()
           self.assertIn('error', json_response)
           self.assertEqual(json_response['error'], 'Request must be JSON')

       def test_check_api_post_bad_json_payload(self):
           app.logger.debug("Running test_check_api_post_bad_json_payload")
           # Send malformed JSON
           bad_json_string = '{"bank_name": "Test Bank", "check_number": "API-1004", "payee_name": "Bad JSON" ...this is not valid json'
           with app.app_context():
                response = self.client.post('/api/v1/check/generate',
                                           data=bad_json_string,
                                           content_type='application/json')
           self.assertEqual(response.status_code, 400)
           self.assertEqual(response.mimetype, 'application/json')
           json_response = response.get_json()
           self.assertIn('error', json_response)
           self.assertEqual(json_response['error'], 'Invalid JSON payload')

if __name__ == '__main__':
    unittest.main()
