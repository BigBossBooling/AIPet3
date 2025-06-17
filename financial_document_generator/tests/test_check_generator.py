import unittest
import os
import pathlib
import datetime # Import datetime
from ..app.check_generator import generate_check

class TestCheckGenerator(unittest.TestCase):

    def setUp(self):
        # Ensure test_output_dir exists
        self.test_output_dir = os.path.join(os.path.dirname(__file__), 'test_outputs')
        if not os.path.exists(self.test_output_dir):
            os.makedirs(self.test_output_dir)

        # Base sample data, individual tests will modify copies
        self.sample_check_data = {
            'bank_name': 'Test Bank United',
            'bank_address': '789 Test Ave, Testville, ST 90000',
            'check_number': '9001',
            'date': datetime.date(2023, 1, 15), # Default date as datetime object
            'payee_name': 'Test Recipient Inc.',
            'amount_numeric': '999.99',
            'amount_words': 'NINE HUNDRED NINETY-NINE AND 99/100',
            'memo': 'Test Memo / For Testing Only',
            'routing_number': '000111222',
            'account_number': '1122334455',
            'bank_logo_url': None,
            'font_configs': [],
            'date_output_format': None
        }

        self.test_pdf_path = os.path.join(self.test_output_dir, 'test_check.pdf') # For general tests

        # Dummy logo for logo tests
        self.dummy_logo_for_test_path = os.path.join(self.test_output_dir, 'test_logo.png')
        try:
            with open(self.dummy_logo_for_test_path, 'w') as f:
                f.write("dummy_logo_content")
        except IOError:
            print(f"Warning: Could not create dummy logo for testing at {self.dummy_logo_for_test_path}")
            self.dummy_logo_for_test_path = None

        # Dummy font for font tests
        self.dummy_font_for_test_path = os.path.join(self.test_output_dir, 'dummy_test_font.ttf')
        try:
            with open(self.dummy_font_for_test_path, 'w') as f:
                f.write("dummy_font_content_for_testing_check_generator")
        except IOError:
            print(f"Warning: Could not create dummy font for testing at {self.dummy_font_for_test_path}")
            self.dummy_font_for_test_path = None


    def tearDown(self):
        # Clean up general test PDF
        if os.path.exists(self.test_pdf_path):
            os.remove(self.test_pdf_path)

        # Clean up dummy logo
        if self.dummy_logo_for_test_path and os.path.exists(self.dummy_logo_for_test_path):
            os.remove(self.dummy_logo_for_test_path)

        # Clean up dummy font
        if self.dummy_font_for_test_path and os.path.exists(self.dummy_font_for_test_path):
            os.remove(self.dummy_font_for_test_path)

        # Clean up test_outputs directory if empty
        if os.path.exists(self.test_output_dir) and not os.listdir(self.test_output_dir):
            try:
                os.rmdir(self.test_output_dir)
            except OSError: # Catch error if dir is somehow not empty or in use
                print(f"Warning: Could not remove test_outputs directory: {self.test_output_dir}")


    def test_generate_check_creates_pdf_file(self):
        '''Test that generate_check creates a PDF file when output_path is provided.'''
        # Use a copy to avoid modifying self.sample_check_data for other tests
        test_data = self.sample_check_data.copy()
        success = generate_check(test_data, output_path=self.test_pdf_path)
        self.assertTrue(success, "generate_check should return True on successful file generation.")
        self.assertTrue(os.path.exists(self.test_pdf_path), "PDF file should be created at the specified output_path.")
        if os.path.exists(self.test_pdf_path):
            self.assertTrue(os.path.getsize(self.test_pdf_path) > 0, "Generated PDF file should not be empty.")

    def test_generate_check_returns_pdf_bytes(self):
        '''Test that generate_check returns PDF content as bytes when output_path is None.'''
        test_data = self.sample_check_data.copy()
        pdf_bytes = generate_check(test_data)
        self.assertIsNotNone(pdf_bytes, "generate_check should return PDF bytes, not None.")
        self.assertIsInstance(pdf_bytes, bytes, "generate_check should return a bytes object.")
        self.assertTrue(len(pdf_bytes) > 0, "Returned PDF bytes should not be empty.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "Returned bytes should start with PDF header '%PDF-'.")

    def test_generate_check_handles_missing_data_gracefully(self):
        incomplete_data = {'payee_name': 'Partial Payee Only'} # Missing many fields
        pdf_bytes = generate_check(incomplete_data)
        self.assertIsNotNone(pdf_bytes, "generate_check should handle missing data by using defaults from template.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "PDF with partial data should still be valid.")

    # --- Logo Tests ---
    def test_generate_check_with_local_logo(self):
        if not self.dummy_logo_for_test_path or not os.path.exists(self.dummy_logo_for_test_path):
            self.skipTest(f"Dummy logo file not available for testing ({self.dummy_logo_for_test_path}).")
        data_with_logo = self.sample_check_data.copy()
        data_with_logo['bank_logo_url'] = self.dummy_logo_for_test_path
        test_logo_pdf_path = os.path.join(self.test_output_dir, 'test_check_with_local_logo.pdf')
        success = generate_check(data_with_logo, output_path=test_logo_pdf_path)
        self.assertTrue(success, "generate_check should return True for local logo.")
        self.assertTrue(os.path.exists(test_logo_pdf_path))
        if os.path.exists(test_logo_pdf_path):
            self.assertTrue(os.path.getsize(test_logo_pdf_path) > 0)
            os.remove(test_logo_pdf_path)

    def test_generate_check_with_remote_logo_url(self):
        data_with_logo = self.sample_check_data.copy()
        data_with_logo['bank_logo_url'] = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII="
        test_logo_pdf_path = os.path.join(self.test_output_dir, 'test_check_with_remote_logo.pdf')
        success = generate_check(data_with_logo, output_path=test_logo_pdf_path)
        self.assertTrue(success, "generate_check should return True for remote logo.")
        self.assertTrue(os.path.exists(test_logo_pdf_path))
        if os.path.exists(test_logo_pdf_path):
            self.assertTrue(os.path.getsize(test_logo_pdf_path) > 0)
            os.remove(test_logo_pdf_path)

    def test_generate_check_with_invalid_local_logo_path(self):
        data_with_logo = self.sample_check_data.copy()
        data_with_logo['bank_logo_url'] = "/path/to/non_existent_logo.png"
        pdf_bytes = generate_check(data_with_logo)
        self.assertIsNotNone(pdf_bytes)
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'))

    # --- Font Tests ---
    def test_generate_check_with_custom_font(self):
        if not self.dummy_font_for_test_path or not os.path.exists(self.dummy_font_for_test_path):
            self.skipTest(f"Dummy font file not available for testing ({self.dummy_font_for_test_path}).")
        data_with_font = self.sample_check_data.copy()
        data_with_font['font_configs'] = [{'font_family': 'MyTestCheckFont', 'font_path': self.dummy_font_for_test_path}]
        test_font_pdf_path = os.path.join(self.test_output_dir, 'test_check_with_custom_font.pdf')
        success = generate_check(data_with_font, output_path=test_font_pdf_path)
        self.assertTrue(success, "generate_check should return True for custom font.")
        self.assertTrue(os.path.exists(test_font_pdf_path))
        if os.path.exists(test_font_pdf_path):
            self.assertTrue(os.path.getsize(test_font_pdf_path) > 0)
            os.remove(test_font_pdf_path)

    def test_generate_check_with_invalid_font_path(self):
        data_with_bad_font = self.sample_check_data.copy()
        data_with_bad_font['font_configs'] = [{'font_family': 'NonExistentFont', 'font_path': '/path/to/non_existent_font.ttf'}]
        pdf_bytes = generate_check(data_with_bad_font)
        self.assertIsNotNone(pdf_bytes)
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'))

    def test_generate_check_with_malformed_font_config_data(self):
        data_malformed_1 = self.sample_check_data.copy()
        data_malformed_1['font_configs'] = "not-a-list"
        pdf_bytes_1 = generate_check(data_malformed_1)
        self.assertIsNotNone(pdf_bytes_1)
        data_malformed_2 = self.sample_check_data.copy()
        data_malformed_2['font_configs'] = [{'font_family': 'MissingPath'}]
        pdf_bytes_2 = generate_check(data_malformed_2)
        self.assertIsNotNone(pdf_bytes_2)

    # --- Date Format Tests ---
    def test_generate_check_with_date_formatting(self):
        '''Test check generation with a specific date_output_format.'''
        data_with_date_fmt = self.sample_check_data.copy()
        data_with_date_fmt['date'] = datetime.date(2023, 3, 20)
        data_with_date_fmt['date_output_format'] = "%m-%d-%Y" # Expected: "03-20-2023"

        pdf_bytes = generate_check(data_with_date_fmt)
        self.assertIsNotNone(pdf_bytes, "PDF should be generated with date formatting options.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "Output should be a valid PDF.")
        # To truly verify the date string, we'd need to inspect PDF content or modify generate_check to return render_data

    def test_generate_check_with_string_date_and_format_option(self):
        '''Test providing a string date when date_output_format is also specified.'''
        data_with_string_date = self.sample_check_data.copy()
        data_with_string_date['date'] = "January 15, 2024" # String date
        data_with_string_date['date_output_format'] = "%Y-%m-%d" # Format option

        pdf_bytes = generate_check(data_with_string_date)
        self.assertIsNotNone(pdf_bytes, "PDF should be generated even if date is string and format is provided.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'))

    def test_generate_check_with_invalid_date_format_string(self):
        '''Test providing an invalid date_output_format string.'''
        data_with_invalid_fmt = self.sample_check_data.copy()
        data_with_invalid_fmt['date'] = datetime.date(2023, 4, 10)
        data_with_invalid_fmt['date_output_format'] = "%Z%Z%ZINVALID" # Invalid format

        pdf_bytes = generate_check(data_with_invalid_fmt)
        self.assertIsNotNone(pdf_bytes, "PDF should be generated even with an invalid date_output_format.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'))

if __name__ == '__main__':
    unittest.main()
