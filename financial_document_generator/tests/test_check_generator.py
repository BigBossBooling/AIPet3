import unittest
import os
# Adjust import path to correctly locate the module
# Assuming 'tests' is a sibling to 'app' directory.
from ..app.check_generator import generate_check

class TestCheckGenerator(unittest.TestCase):

    def setUp(self):
        # Sample data for generating a check
        self.sample_check_data = {
            'bank_name': 'Test Bank United',
            'bank_address': '789 Test Ave, Testville, ST 90000',
            'check_number': '9001',
            'date': 'December 25, 2023',
            'payee_name': 'Test Recipient Inc.',
            'amount_numeric': '999.99',
            'amount_words': 'NINE HUNDRED NINETY-NINE AND 99/100',
            'memo': 'Test Memo / For Testing Only',
            'routing_number': '000111222',
            'account_number': '1122334455'
        }
        # Define a temporary output path for test-generated PDFs
        # This file will be created during the test and cleaned up afterwards.
        self.test_output_dir = os.path.join(os.path.dirname(__file__), 'test_outputs')
        self.test_pdf_path = os.path.join(self.test_output_dir, 'test_check.pdf')

        # Create test_outputs directory if it doesn't exist
        if not os.path.exists(self.test_output_dir):
            os.makedirs(self.test_output_dir)

    def tearDown(self):
        # Clean up: remove the generated PDF file after tests if it exists
        if os.path.exists(self.test_pdf_path):
            os.remove(self.test_pdf_path)
        # Clean up: remove the test_outputs directory if empty
        if os.path.exists(self.test_output_dir) and not os.listdir(self.test_output_dir):
            os.rmdir(self.test_output_dir)


    def test_generate_check_creates_pdf_file(self):
        '''Test that generate_check creates a PDF file when output_path is provided.'''
        success = generate_check(self.sample_check_data, output_path=self.test_pdf_path)
        self.assertTrue(success, "generate_check should return True on successful file generation.")
        self.assertTrue(os.path.exists(self.test_pdf_path), "PDF file should be created at the specified output_path.")
        # Optionally, check if the file size is greater than zero as a basic sanity check
        if os.path.exists(self.test_pdf_path):
            self.assertTrue(os.path.getsize(self.test_pdf_path) > 0, "Generated PDF file should not be empty.")

    def test_generate_check_returns_pdf_bytes(self):
        '''Test that generate_check returns PDF content as bytes when output_path is None.'''
        pdf_bytes = generate_check(self.sample_check_data)
        self.assertIsNotNone(pdf_bytes, "generate_check should return PDF bytes, not None.")
        self.assertIsInstance(pdf_bytes, bytes, "generate_check should return a bytes object.")
        self.assertTrue(len(pdf_bytes) > 0, "Returned PDF bytes should not be empty.")
        # A simple check for PDF header
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "Returned bytes should start with PDF header '%PDF-'.")

    def test_generate_check_handles_missing_data_gracefully(self):
        '''Test how generate_check handles incomplete data (optional, depends on desired behavior).'''
        # This test assumes that the template has defaults for all fields.
        # If not, the function might raise an error or produce an incomplete PDF.
        # For now, we expect it to run due to default values in the template.
        incomplete_data = {'payee_name': 'Partial Payee'}
        pdf_bytes = generate_check(incomplete_data)
        # Check that it still produces a PDF (relying on template defaults)
        self.assertIsNotNone(pdf_bytes, "generate_check should handle missing data by using defaults from template.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "PDF with partial data should still be valid.")


if __name__ == '__main__':
    unittest.main()
